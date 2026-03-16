use serde::{Deserialize, Serialize};

/// Withdrawal requests query payload for `POST /withdrawal-requests/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WithdrawalRequestsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timedelta: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
}

/// Withdrawal request entry returned by `POST /withdrawal-requests/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WithdrawalRequest {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub datetime: String,
    #[serde(rename = "type", default)]
    pub type_field: i64,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub network: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub txid: String,
    #[serde(default)]
    pub address: String,
    #[serde(default)]
    pub transaction_id: String,
}

/// Open bank withdrawal request payload for `POST /withdrawal/open/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenBankWithdrawalRequest {
    pub amount: String,
    pub account_currency: String,
    pub name: String,
    pub iban: String,
    pub bic: String,
    pub address: String,
    pub postal_code: String,
    pub city: String,
    pub country: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intermed_routing_num_or_bic: Option<String>,
}

/// Open bank withdrawal response returned by `POST /withdrawal/open/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenBankWithdrawalResponse {
    #[serde(default)]
    pub withdrawal_id: i64,
}

/// Bank withdrawal status request payload for `POST /withdrawal/status/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BankWithdrawalStatusRequest {
    pub id: String,
}

/// Bank withdrawal status response from `POST /withdrawal/status/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BankWithdrawalStatusResponse {
    #[serde(default)]
    pub status: String,
}

/// Cancel withdrawal request payload for `POST /withdrawal/cancel/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelWithdrawalRequest {
    pub id: String,
}

/// Cancel withdrawal response returned by `POST /withdrawal/cancel/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelWithdrawalResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub account_currency: String,
    #[serde(rename = "type", default)]
    pub type_field: String,
}

/// Crypto withdrawal request payload for `POST /{currency}_withdrawal/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CryptoWithdrawalRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    pub amount: String,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originator_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_thirdparty: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vasp_uuid: Option<String>,
}

/// Crypto withdrawal response returned by `POST /{currency}_withdrawal/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CryptoWithdrawalResponse {
    #[serde(default)]
    pub id: i64,
}

/// Ripple withdrawal request payload for `POST /ripple_withdrawal/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RippleWithdrawalRequest {
    pub currency: String,
    pub amount: String,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originator_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_thirdparty: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vasp_uuid: Option<String>,
}

/// Ripple withdrawal response returned by `POST /ripple_withdrawal/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RippleWithdrawalResponse {
    #[serde(default)]
    pub id: i64,
}
