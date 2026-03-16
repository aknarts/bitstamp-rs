use serde::{Deserialize, Serialize};

/// Sub-account transfer request payload for `POST /transfer-to-main/` and `POST /transfer-from-main/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferRequest {
    pub amount: String,
    pub currency: String,
    #[serde(rename = "subAccount")]
    pub sub_account: i64,
}

/// Sub-account transfer response returned by transfer endpoints.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferResponse {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub reason: String,
}
