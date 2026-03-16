use crate::error::Error;
use crate::types::{Ticker, TickerWithPair};

impl crate::client::Bitstamp {
    /// Returns tickers for all available markets.
    pub async fn get_tickers(&self) -> Result<Vec<TickerWithPair>, Error> {
        self.api_get("ticker/").await
    }

    /// Returns ticker data for a specific market.
    pub async fn get_ticker(&self, currency_pair: &str) -> Result<Ticker, Error> {
        let rest_method = format!("ticker/{}/", currency_pair);
        self.api_get(rest_method.as_str()).await
    }

    /// Returns hourly ticker data for a specific market.
    pub async fn get_hourly_ticker(&self, currency_pair: &str) -> Result<Ticker, Error> {
        let rest_method = format!("ticker_hour/{}/", currency_pair);
        self.api_get(rest_method.as_str()).await
    }
}
