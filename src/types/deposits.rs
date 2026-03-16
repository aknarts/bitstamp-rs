use serde::{Deserialize, Serialize};

/// Deposit address response for `POST /{currency}_address/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DepositAddress {
    #[serde(default)]
    pub address: String,
    #[serde(default)]
    pub memo_id: Option<String>,
    #[serde(default)]
    pub destination_tag: Option<i64>,
    #[serde(default)]
    pub transfer_id: Option<i64>,
}

/// Ripple deposit address response returned by `POST /ripple_address/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RippleDepositAddress {
    #[serde(default)]
    pub address: String,
    #[serde(default)]
    pub destination_tag: Option<i64>,
}
