use crate::error::Error;
use crate::types::{Empty, RevokedApiKeysResponse};

impl crate::client::Bitstamp {
    /// Revokes all API keys. Calls POST /revoke_all_api_keys/.
    pub async fn revoke_all_api_keys(&self) -> Result<RevokedApiKeysResponse, Error> {
        self.api_post("revoke_all_api_keys/", Empty {}).await
    }
}
