use crate::error::Error;
use crate::types::{AccountBalance, Empty};

impl crate::client::Bitstamp {
    /// Get account balances for all currencies
    pub async fn get_account_balances(&self) -> Result<Vec<AccountBalance>, Error> {
        let rest_method = "account_balances/";
        self.api_post(rest_method, Empty {}).await
    }

    /// Get account balance for a specific currency
    pub async fn get_account_balance(&self, currency: &str) -> Result<AccountBalance, Error> {
        let rest_method = format!("account_balances/{}/", currency);
        self.api_post(rest_method.as_str(), Empty {}).await
    }
}
