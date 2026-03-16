use crate::error::Error;
use crate::types::{Time, Transaction};

impl crate::client::Bitstamp {
    /// Returns public transactions for a market. Calls GET /transactions/{pair}/.
    pub async fn get_transactions(&self, currency_pair: &str, time: Option<Time>) -> Result<Vec<Transaction>, Error> {
        let rest_method = format!(
            "transactions/{}/{}",
            currency_pair,
            match time {
                None => "".to_string(),
                Some(t) => format!("?time={}", t.to_string().to_lowercase()),
            }
        );
        self.api_get(rest_method.as_str()).await
    }
}

use crate::types::{
    CryptoDeposit, CryptoTransactionsRequest, CryptoTransactionsResponse, UserTransaction,
    UserTransactionsRequest,
};
use serde_json::Value;

impl crate::client::Bitstamp {
    /// Returns user transactions. Calls POST /user_transactions/.
    pub async fn get_user_transactions(
        &self,
        request: &UserTransactionsRequest,
    ) -> Result<Vec<UserTransaction>, Error> {
        self.api_post("user_transactions/", request).await
    }

    /// Returns user transactions for a market. Calls POST /user_transactions/{market}/.
    pub async fn get_user_transactions_for_market(
        &self,
        market: &str,
        request: &UserTransactionsRequest,
    ) -> Result<Vec<UserTransaction>, Error> {
        let rest_method = format!("user_transactions/{}/", market);
        self.api_post(rest_method.as_str(), request).await
    }

    /// Returns crypto deposits and withdrawals. Calls POST /crypto-transactions/.
    pub async fn get_crypto_transactions(
        &self,
        request: &CryptoTransactionsRequest,
    ) -> Result<CryptoTransactionsResponse, Error> {
        self.api_post("crypto-transactions/", request).await
    }

    /// Returns crypto deposits only. Calls GET /crypto-transactions/deposits/.
    pub async fn get_crypto_deposits(&self) -> Result<Vec<CryptoDeposit>, Error> {
        self.api_auth_get("crypto-transactions/deposits/").await
    }

    /// Updates a crypto deposit. Calls POST /crypto-transactions/deposits/{id}/.
    pub async fn update_crypto_deposit(
        &self,
        id: &str,
        request: Value,
    ) -> Result<Value, Error> {
        let rest_method = format!("crypto-transactions/deposits/{}/", id);
        self.api_post(rest_method.as_str(), request).await
    }
}
