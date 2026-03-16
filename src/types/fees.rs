use serde::{Deserialize, Serialize};

/// Trading fee rate data returned by `POST /fees/trading/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradingFeeRate {
    #[serde(default)]
    pub maker: String,
    #[serde(default)]
    pub taker: String,
}

/// Trading fee entry returned by `POST /fees/trading/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradingFee {
    #[serde(default)]
    pub currency_pair: String,
    #[serde(default)]
    pub market: String,
    #[serde(default)]
    pub fees: TradingFeeRate,
}

/// Withdrawal fee entry returned by `POST /fees/withdrawal/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WithdrawalFee {
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub fee: String,
    #[serde(default)]
    pub network: String,
}

/// Withdrawal fee request payload for `POST /fees/withdrawal/{currency}/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WithdrawalFeeRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
}
