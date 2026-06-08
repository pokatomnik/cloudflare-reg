use std::{error::Error, io};

use base64::{Engine, engine::general_purpose::STANDARD};
use chrono::Utc;
use reqwest::{Proxy, blocking::Client, header::USER_AGENT};
use serde::Serialize;
use serde_json::{Value, json};
use x25519_dalek::{PublicKey, StaticSecret};

const WARP_REGISTRATION_URL: &str = "https://api.cloudflareclient.com/v0a737/reg";

#[derive(Serialize)]
struct RegistrationRequest<'a> {
    key: &'a str,
    install_id: &'static str,
    warp_enabled: bool,
    tos: String,

    #[serde(rename = "type")]
    device_type: &'static str,

    locale: &'static str,
}

fn required_string<'a>(value: &'a Value, field: &str) -> Result<&'a str, io::Error> {
    value[field]
        .as_str()
        .ok_or_else(|| io::Error::other(format!("missing field: {field}")))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut private_key_bytes = StaticSecret::random().to_bytes();

    // WireGuard private-key clamping.
    private_key_bytes[0] &= 248;
    private_key_bytes[31] &= 127;
    private_key_bytes[31] |= 64;

    let private_key = StaticSecret::from(private_key_bytes);
    let public_key = PublicKey::from(&private_key);

    let private_key_base64 = STANDARD.encode(private_key.to_bytes());
    let public_key_base64 = STANDARD.encode(public_key.as_bytes());

    let request = RegistrationRequest {
        key: &public_key_base64,
        install_id: "",
        warp_enabled: true,
        tos: Utc::now().format("%Y-%m-%dT%H:%M:%S.000+00:00").to_string(),
        device_type: "Linux",
        locale: "en_US",
    };

    let response = Client::builder()
        .proxy(Proxy::all("socks5h://127.0.0.1:1080")?)
        .build()?
        .post(WARP_REGISTRATION_URL)
        .header(USER_AGENT, "WARP for Android")
        .json(&request)
        .send()?
        .error_for_status()?
        .json::<Value>()?;

    let peer = &response["config"]["peers"][0];
    let endpoint = &peer["endpoint"];
    let addresses = &response["config"]["interface"]["addresses"];

    let result = json!({
        "private_key": private_key_base64,
        "public_key": public_key_base64,
        "peer_public_key": required_string(peer, "public_key")?,
        "endpoint": required_string(endpoint, "host")?,
        "ipv4": required_string(addresses, "v4")?,
        "ipv6": addresses["v6"].as_str(),
    });

    println!("{}", serde_json::to_string_pretty(&result)?);

    Ok(())
}
