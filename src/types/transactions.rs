use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// User transactions request payload for `POST /user_transactions/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserTransactionsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<String>,
}

/// User transaction entry returned by `POST /user_transactions/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserTransaction {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub datetime: String,
    #[serde(rename = "type", default)]
    pub type_field: String,
    #[serde(default)]
    pub fee: String,
    #[serde(default)]
    pub order_id: Option<i64>,
    #[serde(default)]
    pub self_trade: Option<bool>,
    #[serde(default)]
    pub self_trade_order_id: Option<i64>,
    #[serde(flatten, default)]
    pub other: HashMap<String, Value>,
}

/// Crypto transactions request payload for `POST /crypto-transactions/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CryptoTransactionsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_ious: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_timestamp: Option<String>,
}

/// Crypto transactions response from `POST /crypto-transactions/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CryptoTransactionsResponse {
    #[serde(default)]
    pub deposits: Vec<CryptoDeposit>,
    #[serde(default)]
    pub withdrawals: Vec<CryptoTransaction>,
    #[serde(default)]
    pub ripple_iou_transactions: Vec<CryptoTransaction>,
}

/// Crypto deposit entry returned by `POST /crypto-transactions/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CryptoDeposit {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub network: String,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub destination_address: String,
    #[serde(default)]
    pub txid: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub datetime: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub pending_reason: Option<String>,
}

/// Crypto withdrawal entry returned by `POST /crypto-transactions/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CryptoTransaction {
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub network: String,
    #[serde(rename = "destinationAddress", default)]
    pub destination_address: String,
    #[serde(default)]
    pub txid: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub datetime: String,
}
