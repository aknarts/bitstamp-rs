use serde::{Deserialize, Serialize};

/// Ticker data returned by `GET /ticker/{pair}/` and `GET /ticker_hour/{pair}/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ticker {
    pub high: String,
    pub last: String,
    pub timestamp: String,
    pub bid: String,
    pub vwap: String,
    pub volume: String,
    pub low: String,
    pub ask: String,
    pub open: String,
}

/// Order book data returned by `GET /order_book/{pair}/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderBook {
    pub timestamp: String,
    pub microtimestamp: String,
    pub bids: Vec<Vec<String>>,
    pub asks: Vec<Vec<String>>,
}

/// Public trade data returned by `GET /transactions/{pair}/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub date: String,
    pub tid: String,
    pub price: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub amount: String,
}

/// Time interval for public transactions queries.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Time {
    Minute,
    Hour,
    Day,
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// EUR/USD conversion rate returned by `GET /eur_usd/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConversionRate {
    pub sell: String,
    pub buy: String,
}

/// Empty request payload for POST endpoints without parameters.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Empty {}

/// Offset request payload for paginated endpoints.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Offset {
    pub offset: String,
}

/// Account balance entry returned by `POST /account_balances/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountBalance {
    pub currency: String,
    pub total: String,
    pub available: String,
    pub reserved: String,
}

/// Ticker data with market pair info, returned by `GET /ticker/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TickerWithPair {
    pub high: String,
    pub last: String,
    pub timestamp: String,
    pub bid: String,
    pub vwap: String,
    pub volume: String,
    pub low: String,
    pub ask: String,
    pub open: String,
    pub open_24: String,
    pub percent_change_24: String,
    pub side: String,
    pub pair: String,
    pub market: String,
    #[serde(default)]
    pub market_type: String,
    #[serde(default)]
    pub mark_price: String,
    #[serde(default)]
    pub index_price: String,
    #[serde(default)]
    pub open_interest: String,
    #[serde(default)]
    pub open_interest_value: String,
}

/// Network details for a currency.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrencyNetwork {
    pub network: String,
    #[serde(default)]
    pub withdrawal_minimum_amount: String,
    pub withdrawal_decimals: i64,
    #[serde(default)]
    pub deposit: String,
    #[serde(default)]
    pub withdrawal: String,
}

/// Currency info, returned by `GET /currencies/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Currency {
    pub name: String,
    pub currency: String,
    #[serde(rename = "type")]
    pub currency_type: String,
    #[serde(default)]
    pub symbol: String,
    #[serde(default)]
    pub decimals: i64,
    #[serde(default)]
    pub logo: String,
    #[serde(default)]
    pub available_supply: String,
    #[serde(default)]
    pub deposit: String,
    #[serde(default)]
    pub withdrawal: String,
    #[serde(default)]
    pub networks: Vec<CurrencyNetwork>,
}

/// Market info, returned by `GET /markets/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Market {
    pub name: String,
    pub market_symbol: String,
    pub base_currency: String,
    pub base_decimals: i64,
    pub counter_currency: String,
    pub counter_decimals: i64,
    #[serde(default)]
    pub minimum_order_value: String,
    #[serde(default)]
    pub maximum_order_value: String,
    #[serde(default)]
    pub minimum_order_amount: String,
    #[serde(default)]
    pub maximum_order_amount: String,
    #[serde(default)]
    pub trading: String,
    #[serde(default)]
    pub instant_order_counter_decimals: i64,
    #[serde(default)]
    pub instant_and_market_orders: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub market_type: String,
    #[serde(default)]
    pub underlying_asset: String,
    #[serde(default)]
    pub payoff_type: String,
    #[serde(default)]
    pub contract_size: String,
    #[serde(default)]
    pub tick_size: String,
    #[serde(default)]
    pub isin: Option<String>,
}

/// Single OHLC candle.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OhlcData {
    pub timestamp: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
}

/// Inner OHLC response containing pair and candle data.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OhlcResponse {
    #[serde(default)]
    pub pair: String,
    #[serde(default)]
    pub market: String,
    pub ohlc: Vec<OhlcData>,
}

/// Wrapper for OHLC endpoint response (`{ "data": { ... } }`).
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OhlcResponseWrapper {
    pub data: OhlcResponse,
}

/// Funding rate for a perpetual market.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FundingRate {
    pub funding_rate: String,
    pub timestamp: String,
    pub market: String,
    pub next_funding_time: String,
}

/// Single funding rate tick in history.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FundingRateTick {
    pub funding_rate: String,
    pub timestamp: String,
}

/// Funding rate history response.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FundingRateHistory {
    #[serde(default)]
    pub market: String,
    pub funding_rate_history: Vec<FundingRateTick>,
}
