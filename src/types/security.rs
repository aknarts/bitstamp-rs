use serde::{Deserialize, Serialize};

/// Revoked API keys response from `POST /revoke_all_api_keys/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RevokedApiKeysResponse {
    #[serde(default)]
    pub revoked_api_keys: Vec<String>,
}
