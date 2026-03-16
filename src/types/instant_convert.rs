use serde::{Deserialize, Serialize};

/// Instant convert address info request payload for `POST /instant_convert_address/info/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstantConvertAddressInfoRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

/// Instant convert trade entry returned by address info endpoint.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstantConvertTrade {
    #[serde(default)]
    pub exchange_rate: String,
    #[serde(default)]
    pub btc_amount: String,
    #[serde(default)]
    pub fees: String,
}

/// Instant convert transaction entry returned by address info endpoint.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstantConvertTransaction {
    #[serde(default)]
    pub order_id: i64,
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub trades: Vec<InstantConvertTrade>,
}

/// Instant convert address info response from `POST /instant_convert_address/info/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstantConvertAddressInfoResponse {
    #[serde(default)]
    pub address: String,
    #[serde(default)]
    pub currency_pair: String,
    #[serde(default)]
    pub transactions: Vec<InstantConvertTransaction>,
}

/// New instant convert address request payload for `POST /instant_convert_address/new/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewInstantConvertAddressRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquidation_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_format: Option<String>,
}

/// New instant convert address response from `POST /instant_convert_address/new/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewInstantConvertAddressResponse {
    #[serde(default)]
    pub address: String,
}
