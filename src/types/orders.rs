use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Limit order request payload for `POST /buy/{market}/` and `POST /sell/{market}/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LimitOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    pub price: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_order: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ioc_order: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fok_order: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moc_order: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gtd_datetime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_distance: Option<String>,
}

/// Market order request payload for `POST /buy/market/{market}/` and `POST /sell/market/{market}/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_distance: Option<String>,
}

/// Instant buy order request payload for `POST /buy/instant/{market}/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstantBuyOrderRequest {
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
}

/// Instant sell order request payload for `POST /sell/instant/{market}/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstantSellOrderRequest {
    pub amount: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_in_counter: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
}

/// Cancel order request payload for `POST /cancel_order/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
}

/// Order status request payload for `POST /order_status/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderStatusRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub omit_transactions: Option<String>,
}

/// Order replacement request payload for `POST /replace_order/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReplaceOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orig_client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    pub amount: String,
    pub price: String,
}

/// Order data request payload for `POST /order_data/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderDataRequest {
    pub market: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_id: Option<String>,
}

/// Account order data request payload for `POST /account_order_data/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountOrderDataRequest {
    pub order_source: String,
    pub market: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_id: Option<String>,
}

/// Max order amount request payload for `POST /get_max_order_amount/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaxOrderAmountRequest {
    pub market: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<String>,
}

/// Estimated order impact request payload for `POST /estimated_order_impact/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EstimatedOrderImpactRequest {
    pub market: String,
    pub order_type: String,
    pub amount: String,
    pub order_side: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
}

/// Buy/sell order response from `POST /buy*` and `POST /sell*` endpoints.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuySellOrderResponse {
    pub id: String,
    pub market: String,
    pub datetime: String,
    #[serde(rename = "type")]
    pub order_type: String,
    pub price: String,
    pub amount: String,
    #[serde(default)]
    pub client_order_id: String,
    #[serde(default)]
    pub subtype: String,
    #[serde(default)]
    pub margin_mode: String,
    #[serde(default)]
    pub leverage: String,
    #[serde(default)]
    pub stop_price: String,
    #[serde(default)]
    pub trigger: String,
    #[serde(default)]
    pub reduce_only: bool,
}

/// Cancel order response from `POST /cancel_order/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelOrderResponse {
    pub id: String,
    pub amount: String,
    pub price: String,
    #[serde(rename = "type")]
    pub order_type: String,
    pub market: String,
    pub status: String,
}

/// Cancel all orders response from `POST /cancel_all_orders/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelAllOrdersResponse {
    pub canceled: Vec<CanceledOrder>,
    pub success: bool,
}

/// Canceled order entry returned by cancel-all endpoints.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanceledOrder {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub price: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub order_type: String,
    #[serde(default)]
    pub currency_pair: String,
    #[serde(default)]
    pub market: String,
}

/// Order status response from `POST /order_status/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderStatusResponse {
    #[serde(default)]
    pub id: i64,
    pub datetime: String,
    #[serde(rename = "type")]
    pub order_type: String,
    #[serde(default)]
    pub subtype: String,
    pub status: String,
    pub market: String,
    #[serde(default)]
    pub transactions: Vec<Value>,
    #[serde(default)]
    pub amount_remaining: String,
    #[serde(default)]
    pub client_order_id: String,
}

/// Open order entry returned by `POST /open_orders/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenOrder {
    pub id: String,
    pub datetime: String,
    #[serde(rename = "type")]
    pub order_type: String,
    #[serde(default)]
    pub subtype: String,
    pub price: String,
    pub amount: String,
    #[serde(default)]
    pub amount_at_create: String,
    #[serde(default)]
    pub currency_pair: String,
    #[serde(default)]
    pub market: String,
    #[serde(default)]
    pub client_order_id: String,
}

/// Order replacement response from `POST /replace_order/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReplaceOrderResponse {
    #[serde(default)]
    pub order_id: i64,
    #[serde(default)]
    pub order_type: String,
    pub market: String,
    pub amount: String,
    pub price: String,
    pub datetime: String,
    #[serde(default)]
    pub orig_order_id: i64,
    #[serde(default)]
    pub orig_client_order_id: Option<String>,
    #[serde(default)]
    pub status: String,
}

/// Max order amount response from `POST /get_max_order_amount/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaxOrderAmount {
    #[serde(default)]
    pub maximum_order_amount: String,
    #[serde(default)]
    pub maximum_order_value: String,
    pub maximum_order_amount_currency: String,
    pub maximum_order_value_currency: String,
}

/// Trading pair entry returned by `GET /my_markets/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradingPair {
    pub name: String,
    pub url_symbol: String,
}

/// Order event entry returned by `POST /order_data/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderEvent {
    pub event: String,
    pub event_id: String,
    #[serde(default)]
    pub order_source: String,
    #[serde(default)]
    pub data: Value,
}

/// Account order event entry returned by `POST /account_order_data/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountOrderEvent {
    pub event: String,
    pub event_id: String,
    #[serde(default)]
    pub order_source: String,
    #[serde(default)]
    pub trade_account_id: i64,
    #[serde(default)]
    pub data: Value,
}

/// Estimated order impact response from `POST /estimated_order_impact/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EstimatedOrderImpact {
    #[serde(default)]
    pub margin_currency: String,
    #[serde(default)]
    pub additional_required_margin: String,
    #[serde(default)]
    pub margin_required_for_order: Option<String>,
    #[serde(default)]
    pub estimated_liquidation_prices: Value,
}
