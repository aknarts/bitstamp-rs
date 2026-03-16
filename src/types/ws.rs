use serde::de;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

/// Incoming WebSocket event message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub event: EventEvent,
    pub channel: EventChannel,
    pub data: EventData,
}

/// WebSocket event name.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventEvent {
    #[serde(rename = "bts:subscribe")]
    BtsSubscribe,
    #[serde(rename = "bts:unsubscribe")]
    BtsUnsubscribe,
    #[serde(rename = "bts:subscription_succeeded")]
    BtsSubscriptionSucceeded,
    #[serde(rename = "bts:unsubscription_succeeded")]
    BtsUnsubscriptionSucceeded,
    #[serde(rename = "bts:request_reconnect")]
    BtsRequestReconnect,
    Trade,
    OrderCreated,
    OrderChanged,
    OrderDeleted,
    Data,
}

/// Outgoing WebSocket event message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutEvent {
    pub event: EventEvent,
    pub data: OutEventData,
}

/// Outgoing WebSocket event data payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutEventData {
    pub channel: EventChannel,
}

/// WebSocket event data payload.
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

/// WebSocket channel selector for subscriptions.
#[derive(Debug, Clone)]
pub enum EventChannel {
    LiveTrades(CurrencyPair),
    LiveOrders(CurrencyPair),
    OrderBook(CurrencyPair),
    DetailOrderBook(CurrencyPair),
    DiffOrderBook(CurrencyPair),
}

impl Serialize for EventChannel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            EventChannel::LiveTrades(pair) => {
                serializer.serialize_str(&format!("live_trades_{}", pair))
            }
            EventChannel::LiveOrders(pair) => {
                serializer.serialize_str(&format!("live_orders_{}", pair))
            }
            EventChannel::OrderBook(pair) => {
                serializer.serialize_str(&format!("order_book_{}", pair))
            }
            EventChannel::DetailOrderBook(pair) => {
                serializer.serialize_str(&format!("detail_order_book_{}", pair))
            }
            EventChannel::DiffOrderBook(pair) => {
                serializer.serialize_str(&format!("diff_order_book_{}", pair))
            }
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
                let pair: CurrencyPair = match parts.last() {
                    None => {
                        return Err(de::Error::custom("Failed to parse CurrencyPair"));
                    }
                    Some(p) => match CurrencyPair::from_str(p) {
                        Ok(pa) => pa,
                        Err(_) => {
                            return Err(de::Error::custom(format!("Unknown currency pair: {}", p)));
                        }
                    },
                };
                parts.truncate(parts.len().saturating_sub(1));
                match parts.join("_").as_str() {
                    "live_trades" => Ok(EventChannel::LiveTrades(pair)),
                    "live_orders" => Ok(EventChannel::LiveOrders(pair)),
                    "order_book" => Ok(EventChannel::OrderBook(pair)),
                    "detail_order_book" => Ok(EventChannel::DetailOrderBook(pair)),
                    "diff_order_book" => Ok(EventChannel::DiffOrderBook(pair)),
                    _ => Err(de::Error::custom("Unknown channel")),
                }
            }

            fn visit_string<E: de::Error>(self, s: String) -> Result<Self::Value, E> {
                self.visit_str(s.as_str())
            }
        }

        deserializer.deserialize_any(EventChannelVisitor)
    }
}

/// Currency pair identifier used in WebSocket channels.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CurrencyPair(pub String);

impl CurrencyPair {
    /// Creates a new lowercased currency pair.
    pub fn new(pair: &str) -> Self {
        CurrencyPair(pair.to_lowercase())
    }

    /// Returns the currency pair as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for CurrencyPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for CurrencyPair {
    type Err = ();

    fn from_str(s: &str) -> Result<CurrencyPair, ()> {
        if s.is_empty() {
            Err(())
        } else {
            Ok(CurrencyPair(s.to_lowercase()))
        }
    }
}

impl Serialize for CurrencyPair {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for CurrencyPair {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(CurrencyPair(s.to_lowercase()))
    }
}
