use serde::{Deserialize, Serialize, Serializer, Deserializer};
use serde::de::{self, Visitor, Error};
use std::str::FromStr;

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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderBook {
    pub timestamp: String,
    pub microtimestamp: String,
    pub bids: Vec<Vec<String>>,
    pub asks: Vec<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub date: String,
    pub tid: String,
    pub price: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub amount: String,
}

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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PairInfo {
    pub base_decimals: i64,
    pub minimum_order: String,
    pub name: String,
    pub counter_decimals: i64,
    pub trading: String,
    pub url_symbol: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConversionRate {
    pub sell: String,
    pub buy: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Empty {}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Offset {
    pub offset: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountBalance {
    pub bch_available: String,
    pub bch_balance: String,
    pub bch_reserved: String,
    pub bch_withdrawal_fee: String,
    pub bchbtc_fee: String,
    pub bcheur_fee: String,
    pub bchusd_fee: String,
    pub btc_available: String,
    pub btc_balance: String,
    pub btc_reserved: String,
    pub btc_withdrawal_fee: String,
    pub btceur_fee: String,
    pub btcusd_fee: String,
    pub eth_available: String,
    pub eth_balance: String,
    pub eth_reserved: String,
    pub eth_withdrawal_fee: String,
    pub ethbtc_fee: String,
    pub etheur_fee: String,
    pub ethusd_fee: String,
    pub eur_available: String,
    pub eur_balance: String,
    pub eur_reserved: String,
    pub eurusd_fee: String,
    pub ltc_available: String,
    pub ltc_balance: String,
    pub ltc_reserved: String,
    pub ltc_withdrawal_fee: String,
    pub ltcbtc_fee: String,
    pub ltceur_fee: String,
    pub ltcusd_fee: String,
    pub usd_available: String,
    pub usd_balance: String,
    pub usd_reserved: String,
    pub xrp_available: String,
    pub xrp_balance: String,
    pub xrp_reserved: String,
    pub xrp_withdrawal_fee: String,
    pub xrpbtc_fee: String,
    pub xrpeur_fee: String,
    pub xrpusd_fee: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct V2Error {
    pub status: String,
    pub reason: String,
    pub code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct V1Error {
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub event: EventEvent,
    pub channel: EventChannel,
    pub data: EventData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventEvent {
    #[serde(rename = "bts:subscribe")]
    BtsSubscribe,
    #[serde(rename = "bts:unsubscribe")]
    BtsUnsubscribe,
    Trade,
    OrderCreated,
    OrderChanged,
    OrderDeleted,
    Data,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutEvent {
    pub event: EventEvent,
    pub data: OutEventData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutEventData {
    pub channel: EventChannel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventData {
    Trade {
        buy_order_id: i64,
        amount_str: String,
        timestamp: String,
        microtimestamp: String,
        id: i64,
        amount: f64,
        sell_order_id: i64,
        price_str: String,
        #[serde(rename = "type")]
        type_field: i64,
        price: f64,
    },
    Orders {
        id: i64,
        id_str: String,
        order_type: i64,
        datetime: String,
        microtimestamp: String,
        amount: f64,
        amount_str: String,
        price: f64,
        price_str: String,
    },
    OrderBook {
        timestamp: String,
        microtimestamp: String,
        bids: Vec<Vec<String>>,
        asks: Vec<Vec<String>>,
    },
    Empty {},
}

#[derive(Debug, Clone)]
pub enum EventChannel {
    LiveTrades(CurrencyPairs),
    LiveOrders(CurrencyPairs),
    OrderBook(CurrencyPairs),
    DetailOrderBook(CurrencyPairs),
    DiffOrderBook(CurrencyPairs),
}

impl Serialize for EventChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match self {
            EventChannel::LiveTrades(pair) => {
                serializer.serialize_str(format!("live_trades_{:?}", pair).to_lowercase().as_str())
            }
            EventChannel::LiveOrders(pair) => { serializer.serialize_str(format!("live_orders_{:?}", pair).to_lowercase().as_str()) }
            EventChannel::OrderBook(pair) => { serializer.serialize_str(format!("order_book_{:?}", pair).to_lowercase().as_str()) }
            EventChannel::DetailOrderBook(pair) => { serializer.serialize_str(format!("detail_order_book_{:?}", pair).to_lowercase().as_str()) }
            EventChannel::DiffOrderBook(pair) => { serializer.serialize_str(format!("diff_order_book_{:?}", pair).to_lowercase().as_str()) }
        }
    }
}

impl<'de> Deserialize<'de> for EventChannel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct EventChannelVisitor;
        impl<'de> de::Visitor<'de> for EventChannelVisitor {
            type Value = EventChannel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "an event channel")
            }

            fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
                let mut parts: Vec<&str> = s.split('_').collect();
                let pair: CurrencyPairs = match parts.last() {
                    None => { return Err(de::Error::custom("Failed to parse CurrencyPairs")); }
                    Some(p) => {
                        match CurrencyPairs::from_str(p) {
                            Ok(pa) => { pa }
                            Err(e) => { return Err(de::Error::custom(format!("Unknown currency pair: {}", p))); }
                        }
                    }
                };
                parts.truncate(parts.len().saturating_sub(1));
                match parts.join("_").as_str() {
                    "live_trades" => { Ok(EventChannel::LiveTrades(pair)) }
                    "live_orders" => { Ok(EventChannel::LiveOrders(pair)) }
                    "order_book" => { Ok(EventChannel::OrderBook(pair)) }
                    "detail_order_book" => { Ok(EventChannel::DetailOrderBook(pair)) }
                    "diff_order_book" => { Ok(EventChannel::DiffOrderBook(pair)) }
                    _ => {
                        Err(de::Error::custom("Unknown channel"))
                    }
                }
            }


            fn visit_string<E: de::Error>(self, s: String) -> Result<Self::Value, E> {
                self.visit_str(s.as_str())
            }
        }

        deserializer.deserialize_any(EventChannelVisitor)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CurrencyPairs {
    Btcusd,
    Btceur,
    Eurusd,
    Xrpusd,
    Xrpeur,
    Xrpbtc,
    Ltcusd,
    Ltceur,
    Ltcbtc,
    Ethusd,
    Etheur,
    Ethbtc,
    Bchusd,
    Bcheur,
    Bchbtc,
}

impl FromStr for CurrencyPairs {
    type Err = ();

    fn from_str(s: &str) -> Result<CurrencyPairs, ()> {
        match s.to_lowercase().as_str() {
            "btcusd" => Ok(CurrencyPairs::Btcusd),
            "btceur" => Ok(CurrencyPairs::Btceur),
            "eurusd" => Ok(CurrencyPairs::Eurusd),
            "xrpusd" => Ok(CurrencyPairs::Xrpusd),
            "xrpeur" => Ok(CurrencyPairs::Xrpeur),
            "xrpbtc" => Ok(CurrencyPairs::Xrpbtc),
            "ltcusd" => Ok(CurrencyPairs::Ltcusd),
            "ltceur" => Ok(CurrencyPairs::Ltceur),
            "ltcbtc" => Ok(CurrencyPairs::Ltcbtc),
            "ethusd" => Ok(CurrencyPairs::Ethusd),
            "etheur" => Ok(CurrencyPairs::Etheur),
            "ethbtc" => Ok(CurrencyPairs::Ethbtc),
            "bchusd" => Ok(CurrencyPairs::Bchusd),
            "bcheur" => Ok(CurrencyPairs::Bcheur),
            "bchbtc" => Ok(CurrencyPairs::Bchbtc),
            _ => Err(()),
        }
    }
}