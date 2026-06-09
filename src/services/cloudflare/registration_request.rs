use serde::Serialize;

/// Compile‑time constants used for the registration payload.
pub(crate) const INSTALL_ID: &str = "";
pub(crate) const WARP_ENABLED: bool = true;
pub(crate) const DEVICE_TYPE: &str = "Linux";
pub(crate) const LOCALE: &str = "en_US";

#[derive(Serialize)]
pub(crate) struct RegistrationRequest<'a> {
    key: &'a str,
    install_id: &'static str,
    warp_enabled: bool,
    tos: String,

    #[serde(rename = "type")]
    device_type: &'static str,

    locale: &'static str,
}

impl<'a> RegistrationRequest<'a> {
    /// Creates a new `RegistrationRequest`.
    ///
    /// Only the values that are only known at runtime are required as arguments.
    /// The remaining fields are filled with compile‑time constants defined above.
    pub fn new(key: &'a str, tos: String) -> Self {
        Self {
            key,
            install_id: INSTALL_ID,
            warp_enabled: WARP_ENABLED,
            tos,
            device_type: DEVICE_TYPE,
            locale: LOCALE,
        }
    }
}
