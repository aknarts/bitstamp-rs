use crate::error::Error;
use crate::types::{Empty, TradingFee, WithdrawalFee, WithdrawalFeeRequest};

impl crate::client::Bitstamp {
    /// Returns trading fees for all markets. Calls POST /fees/trading/.
    pub async fn get_trading_fees(&self) -> Result<Vec<TradingFee>, Error> {
        self.api_post("fees/trading/", Empty {}).await
    }

    /// Returns trading fees for a market. Calls POST /fees/trading/{market}/.
    pub async fn get_trading_fee(&self, market: &str) -> Result<TradingFee, Error> {
        let rest_method = format!("fees/trading/{}/", market);
        self.api_post(rest_method.as_str(), Empty {}).await
    }

    /// Returns withdrawal fees for all currencies. Calls POST /fees/withdrawal/.
    pub async fn get_withdrawal_fees(&self) -> Result<Vec<WithdrawalFee>, Error> {
        self.api_post("fees/withdrawal/", Empty {}).await
    }

    /// Returns withdrawal fee for a currency. Calls POST /fees/withdrawal/{currency}/.
    pub async fn get_withdrawal_fee(
        &self,
        currency: &str,
        network: Option<&str>,
    ) -> Result<WithdrawalFee, Error> {
        let rest_method = format!("fees/withdrawal/{}/", currency);
        let request = WithdrawalFeeRequest {
            network: network.map(String::from),
        };
        self.api_post(rest_method.as_str(), request).await
    }
}
