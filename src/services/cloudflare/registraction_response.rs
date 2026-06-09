use std::fmt::Display;

pub(crate) struct RegistrationResponse {
    private_key: String,
    public_key: String,
    peer_public_key: String,
    endpoint: String,
    ipv4: String,
    ipv6: Option<String>,
}

impl RegistrationResponse {
    pub fn new(
        private_key: String,
        public_key: String,
        peer_public_key: String,
        endpoint: String,
        ipv4: String,
        ipv6: Option<String>,
    ) -> Self {
        Self {
            private_key,
            public_key,
            peer_public_key,
            endpoint,
            ipv4,
            ipv6,
        }
    }

    pub fn private_key(&self) -> &str {
        &self.private_key
    }

    pub fn public_key(&self) -> &str {
        &self.public_key
    }

    pub fn peer_public_key(&self) -> &str {
        &self.peer_public_key
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn ipv4(&self) -> &str {
        &self.ipv4
    }

    pub fn ipv6(&self) -> Option<&str> {
        self.ipv6.as_deref()
    }
}

impl Display for RegistrationResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::from("Cloudflare registration\n\n");
        let ips_str = self
            .ipv6()
            .map(|ipv6| format!("{}/0,{}/0", self.ipv4(), ipv6))
            .unwrap_or_else(|| self.ipv4().to_string());

        buf.push_str(format!("Private key*: {}\n", self.private_key()).as_str());
        buf.push_str(format!("Public key: {}\n", self.public_key()).as_str());
        buf.push_str(format!("Peer public key*: {}\n", self.peer_public_key()).as_str());
        buf.push_str(format!("IP addresses*: {}\n", ips_str).as_str());
        buf.push_str(format!("Endpoint*: {}\n\n", self.endpoint()).as_str());

        buf.push_str("Values with * must be used in your client\n");

        f.write_str(&buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt_with_ipv6() {
        let resp = RegistrationResponse::new(
            "priv_key".to_string(),
            "pub_key".to_string(),
            "peer_key".to_string(),
            "example.com:1234".to_string(),
            "10.0.0.1".to_string(),
            Some("fd00::1".to_string()),
        );
        let formatted = format!("{}", resp);
        let expected = "\
Cloudflare registration\n\n\
Private key*: priv_key\n\
Public key: pub_key\n\
Peer public key*: peer_key\n\
IP addresses*: 10.0.0.1/0,fd00::1/0\n\
Endpoint*: example.com:1234\n\n\
Values with * must be used in your client\n";
        assert_eq!(formatted, expected);
    }
}
