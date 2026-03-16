use crate::error::Error;
use crate::types::{EarnSubscribeRequest, EarnSubscription, EarnSubscriptionSettingRequest, EarnTransaction};
use serde_json::Value;

impl crate::client::Bitstamp {
    /// Subscribes to an earn product. Calls POST /earn/subscribe/.
    pub async fn earn_subscribe(&self, request: &EarnSubscribeRequest) -> Result<Value, Error> {
        self.api_post("earn/subscribe/", request).await
    }

    /// Unsubscribes from an earn product. Calls POST /earn/unsubscribe/.
    pub async fn earn_unsubscribe(&self, request: &EarnSubscribeRequest) -> Result<Value, Error> {
        self.api_post("earn/unsubscribe/", request).await
    }

    /// Returns earn subscriptions. Calls GET /earn/subscriptions/.
    pub async fn get_earn_subscriptions(&self) -> Result<Vec<EarnSubscription>, Error> {
        self.api_auth_get("earn/subscriptions/").await
    }

    /// Updates earn subscription setting. Calls POST /earn/subscriptions/setting/.
    pub async fn manage_earn_subscription_setting(
        &self,
        request: &EarnSubscriptionSettingRequest,
    ) -> Result<Value, Error> {
        self.api_post("earn/subscriptions/setting/", request).await
    }

    /// Returns earn transactions. Calls GET /earn/transactions/.
    pub async fn get_earn_transactions(&self) -> Result<Vec<EarnTransaction>, Error> {
        self.api_auth_get("earn/transactions/").await
    }
}
