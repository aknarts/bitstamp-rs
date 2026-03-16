use serde::{Deserialize, Serialize};

/// V2 error response returned by Bitstamp API.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct V2Error {
    pub status: String,
    pub reason: String,
    pub code: String,
}

/// V1 error response returned by Bitstamp API.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct V1Error {
    pub error: String,
}
