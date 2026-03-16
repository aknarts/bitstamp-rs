use crate::error::Error;
use crate::types::{
    ConversionRate, Currency, FundingRate, FundingRateHistory, Market,
    OhlcResponseWrapper,
};

impl crate::client::Bitstamp {
    /// Returns EUR/USD conversion rate.
    pub async fn get_eur_usd(&self) -> Result<ConversionRate, Error> {
        let rest_method = "eur_usd/";
        self.api_get(rest_method).await
    }

    /// Returns a list of all supported currencies.
    pub async fn get_currencies(&self) -> Result<Vec<Currency>, Error> {
        self.api_get("currencies/").await
    }

    /// Returns a list of all available markets with their details.
    pub async fn get_markets(&self) -> Result<Vec<Market>, Error> {
        self.api_get("markets/").await
    }

    /// Returns OHLC data for a market.
    ///
    /// # Arguments
    /// * `market` — Market symbol (e.g. `"btcusd"`)
    /// * `step` — Time interval in seconds (60, 180, 300, 900, 1800, 3600, 7200, 14400, 21600, 43200, 86400, 259200)
    /// * `limit` — Number of candles to return
    /// * `start` — Optional unix timestamp for the start of the range
    /// * `end` — Optional unix timestamp for the end of the range
    /// * `exclude_current_candle` — Whether to exclude the current (incomplete) candle
    pub async fn get_ohlc(
        &self,
        market: &str,
        step: u32,
        limit: u32,
        start: Option<u64>,
        end: Option<u64>,
        exclude_current_candle: Option<bool>,
    ) -> Result<OhlcResponseWrapper, Error> {
        let mut query = format!("?step={}&limit={}", step, limit);
        if let Some(s) = start {
            query.push_str(&format!("&start={}", s));
        }
        if let Some(e) = end {
            query.push_str(&format!("&end={}", e));
        }
        if let Some(exc) = exclude_current_candle {
            query.push_str(&format!("&exclude_current_candle={}", exc));
        }
        let rest_method = format!("ohlc/{}/{}", market, query);
        self.api_get(rest_method.as_str()).await
    }

    /// Returns the current funding rate for a perpetual market.
    pub async fn get_funding_rate(&self, market: &str) -> Result<FundingRate, Error> {
        let rest_method = format!("funding_rate/{}/", market);
        self.api_get(rest_method.as_str()).await
    }

    /// Returns funding rate history for a perpetual market.
    ///
    /// # Arguments
    /// * `market` — Market symbol (e.g. `"btcusd-perp"`)
    /// * `limit` — Max number of entries to return
    /// * `since_timestamp` — Only return entries after this unix timestamp
    /// * `until_timestamp` — Only return entries before this unix timestamp
    pub async fn get_funding_rate_history(
        &self,
        market: &str,
        limit: Option<u32>,
        since_timestamp: Option<u64>,
        until_timestamp: Option<u64>,
    ) -> Result<FundingRateHistory, Error> {
        let mut params = Vec::new();
        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }
        if let Some(s) = since_timestamp {
            params.push(format!("since_timestamp={}", s));
        }
        if let Some(u) = until_timestamp {
            params.push(format!("until_timestamp={}", u));
        }
        let query = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        let rest_method = format!("funding_rate_history/{}/{}", market, query);
        self.api_get(rest_method.as_str()).await
    }
}
