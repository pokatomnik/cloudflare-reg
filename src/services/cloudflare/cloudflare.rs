use base64::{Engine, engine::general_purpose::STANDARD};
use chrono::Utc;
use reqwest::{Proxy, blocking::Client, header::USER_AGENT};
use serde_json::Value;
use x25519_dalek::{PublicKey, StaticSecret};

use crate::{
    services::cloudflare::{
        registraction_response::RegistrationResponse, registration_request::RegistrationRequest,
    },
    utils::value_ext::ValueExt,
};

const WARP_REGISTRATION_URL: &str = "https://api.cloudflareclient.com/v0a737/reg";
const USER_AGENT_VALUE: &str = "WARP for Android";

pub(crate) struct Cloudflare {
    proxy: Option<Proxy>,
}

impl Cloudflare {
    pub fn new(proxy_scheme: Option<impl AsRef<str>>) -> Self {
        let proxy = proxy_scheme.and_then(|s| Proxy::all(s.as_ref().to_string()).ok());
        Self { proxy }
    }

    fn get_key(&self) -> [u8; 32] {
        let mut private_key_bytes = StaticSecret::random().to_bytes();

        // WireGuard private-key clamping.
        private_key_bytes[0] &= 248;
        private_key_bytes[31] &= 127;
        private_key_bytes[31] |= 64;

        private_key_bytes
    }

    fn get_private_key(&self, private_key_bytes: [u8; 32]) -> StaticSecret {
        StaticSecret::from(private_key_bytes)
    }

    fn get_public_key(&self, private_key: &StaticSecret) -> PublicKey {
        PublicKey::from(private_key)
    }

    fn private_key_base64(&self, private_key: &StaticSecret) -> String {
        STANDARD.encode(private_key.to_bytes())
    }

    fn public_key_base64(&self, public_key: &PublicKey) -> String {
        STANDARD.encode(public_key.as_bytes())
    }

    fn get_client(&self) -> anyhow::Result<Client> {
        let mut builder = Client::builder();
        if let Some(ref proxy) = self.proxy {
            builder = builder.proxy(proxy.to_owned());
        }

        let builder = builder.build()?;
        Ok(builder)
    }

    pub fn register(&self) -> anyhow::Result<RegistrationResponse> {
        let private_key_bytes = self.get_key();

        let private_key = self.get_private_key(private_key_bytes);
        let public_key = self.get_public_key(&private_key);

        let private_key_base64 = self.private_key_base64(&private_key);
        let public_key_base64 = self.public_key_base64(&public_key);

        let request = RegistrationRequest::new(
            &public_key_base64,
            Utc::now().format("%Y-%m-%dT%H:%M:%S.000+00:00").to_string(),
        );

        let client = self.get_client()?;

        let request = client
            .post(WARP_REGISTRATION_URL)
            .header(USER_AGENT, USER_AGENT_VALUE)
            .json(&request);

        let response = request.send()?.error_for_status()?.json::<Value>()?;

        let peer = (&response)
            .value_by_path(&["config", "peers"])
            .and_then(|v| v.get(0))
            .map(|v| v.clone())
            .ok_or_else(|| anyhow::Error::msg("Incorrect Cloudflare response: missing peers"))?;
        let endpoint = (&peer)
            .value_by_path(&["endpoint"])
            .map(|v| v.clone())
            .ok_or_else(|| anyhow::Error::msg("Incorrect Cloudflare response: missing endpoint"))?;
        let addresses = (&response)
            .value_by_path(&["config", "interface", "addresses"])
            .map(|v| v.clone())
            .ok_or_else(|| {
                anyhow::Error::msg("Incorrect Cloudflare response: missing addresses")
            })?;

        let registration_response = RegistrationResponse::new(
            private_key_base64,
            public_key_base64,
            (&peer).required_str("public_key")?.to_string(),
            (&endpoint).required_str("host")?.to_string(),
            (&addresses).required_str("v4")?.to_string(),
            (&addresses).str("v6").map(ToString::to_string),
        );

        Ok(registration_response)
    }
}
