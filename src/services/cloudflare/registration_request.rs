use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct RegistrationRequest<'a> {
    pub key: &'a str,
    pub install_id: &'static str,
    pub warp_enabled: bool,
    pub tos: String,

    #[serde(rename = "type")]
    pub device_type: &'static str,

    pub locale: &'static str,
}
