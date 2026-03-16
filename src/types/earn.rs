use serde::{Deserialize, Serialize};

/// Earn subscription request payload for `POST /earn/subscribe/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EarnSubscribeRequest {
    pub currency: String,
    pub earn_type: String,
    pub earn_term: String,
    pub amount: String,
}

/// Earn subscription setting request payload for `POST /earn/subscriptions/setting/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EarnSubscriptionSettingRequest {
    pub setting: String,
    pub currency: String,
    pub earn_type: String,
}

/// Earn subscription entry returned by `GET /earn/subscriptions/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EarnSubscription {
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(rename = "type", default)]
    pub type_field: Option<String>,
    #[serde(default)]
    pub term: Option<String>,
    #[serde(default)]
    pub estimated_annual_yield: f64,
    #[serde(default)]
    pub distribution_period: Option<String>,
    #[serde(default)]
    pub activation_period: Option<String>,
    #[serde(default)]
    pub minimum_subscription_amount: f64,
    #[serde(default)]
    pub amount: f64,
    #[serde(default)]
    pub available_amount: f64,
    #[serde(default)]
    pub amount_earned: f64,
}

/// Earn transaction entry returned by `GET /earn/transactions/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EarnTransaction {
    #[serde(default)]
    pub datetime: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub value: String,
    #[serde(default)]
    pub quote_currency: String,
    #[serde(default)]
    pub status: String,
}
