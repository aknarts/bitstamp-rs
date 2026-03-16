use crate::error::Error;
use crate::types::{DepositAddress, Empty, RippleDepositAddress};
use serde_json::Value;

impl crate::client::Bitstamp {
    /// Returns a crypto deposit address. Calls POST /{currency}_address/.
    pub async fn get_crypto_deposit_address(&self, currency: &str) -> Result<DepositAddress, Error> {
        let rest_method = format!("{}_address/", currency);
        self.api_post(rest_method.as_str(), Empty {}).await
    }

    /// Returns unconfirmed BTC transactions. Calls POST /btc_unconfirmed/.
    pub async fn get_btc_unconfirmed(&self) -> Result<Value, Error> {
        self.api_post("btc_unconfirmed/", Empty {}).await
    }

    /// Returns the Ripple deposit address. Calls POST /ripple_address/.
    pub async fn get_ripple_address(&self) -> Result<RippleDepositAddress, Error> {
        self.api_post("ripple_address/", Empty {}).await
    }
}
