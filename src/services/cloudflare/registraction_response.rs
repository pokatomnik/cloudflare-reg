use std::fmt::Display;

pub(crate) struct RegistrationResponse {
    pub private_key: String,
    pub public_key: String,
    pub peer_public_key: String,
    pub endpoint: String,
    pub ipv4: String,
    pub ipv6: Option<String>,
}

impl Display for RegistrationResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::from("Cloudflare registration\n\n");
        let ips_str = self
            .ipv6
            .as_ref()
            .map(|ipv6| format!("{},{}", &self.ipv4.clone(), ipv6))
            .unwrap_or((&self.ipv4).clone());

        buf.push_str(format!("Private key*: {}\n", self.private_key).as_str());
        buf.push_str(format!("Public key: {}\n", self.public_key).as_str());
        buf.push_str(format!("Peer public key*: {}\n", self.peer_public_key).as_str());
        buf.push_str(format!("IP addresses*: {}\n", ips_str).as_str());
        buf.push_str(format!("Endpoint*: {}\n\n", self.endpoint).as_str());

        buf.push_str("Values with * must be used in your client\n");

        f.write_str(buf.as_str())
    }
}
