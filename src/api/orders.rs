use crate::error::Error;
use crate::types::{
    AccountOrderDataRequest, AccountOrderEvent, BuySellOrderResponse, CancelAllOrdersResponse,
    CancelOrderRequest, CancelOrderResponse, Empty, EstimatedOrderImpact,
    EstimatedOrderImpactRequest, InstantBuyOrderRequest, InstantSellOrderRequest,
    LimitOrderRequest, MarketOrderRequest, MaxOrderAmount, MaxOrderAmountRequest, OpenOrder,
    OrderDataRequest, OrderEvent, OrderStatusRequest, OrderStatusResponse, ReplaceOrderRequest,
    ReplaceOrderResponse, TradingPair,
};

impl crate::client::Bitstamp {
    /// Places a limit buy order. Calls POST /buy/{market}/.
    pub async fn buy_limit_order(
        &self,
        market: &str,
        request: &LimitOrderRequest,
    ) -> Result<BuySellOrderResponse, Error> {
        let rest_method = format!("buy/{}/", market);
        self.api_post(rest_method.as_str(), request).await
    }

    /// Places a market buy order. Calls POST /buy/market/{market}/.
    pub async fn buy_market_order(
        &self,
        market: &str,
        request: &MarketOrderRequest,
    ) -> Result<BuySellOrderResponse, Error> {
        let rest_method = format!("buy/market/{}/", market);
        self.api_post(rest_method.as_str(), request).await
    }

    /// Places an instant buy order. Calls POST /buy/instant/{market}/.
    pub async fn buy_instant_order(
        &self,
        market: &str,
        request: &InstantBuyOrderRequest,
    ) -> Result<BuySellOrderResponse, Error> {
        let rest_method = format!("buy/instant/{}/", market);
        self.api_post(rest_method.as_str(), request).await
    }

    /// Places a limit sell order. Calls POST /sell/{market}/.
    pub async fn sell_limit_order(
        &self,
        market: &str,
        request: &LimitOrderRequest,
    ) -> Result<BuySellOrderResponse, Error> {
        let rest_method = format!("sell/{}/", market);
        self.api_post(rest_method.as_str(), request).await
    }

    /// Places a market sell order. Calls POST /sell/market/{market}/.
    pub async fn sell_market_order(
        &self,
        market: &str,
        request: &MarketOrderRequest,
    ) -> Result<BuySellOrderResponse, Error> {
        let rest_method = format!("sell/market/{}/", market);
        self.api_post(rest_method.as_str(), request).await
    }

    /// Places an instant sell order. Calls POST /sell/instant/{market}/.
    pub async fn sell_instant_order(
        &self,
        market: &str,
        request: &InstantSellOrderRequest,
    ) -> Result<BuySellOrderResponse, Error> {
        let rest_method = format!("sell/instant/{}/", market);
        self.api_post(rest_method.as_str(), request).await
    }

    /// Cancels a single order. Calls POST /cancel_order/.
    pub async fn cancel_order(
        &self,
        request: &CancelOrderRequest,
    ) -> Result<CancelOrderResponse, Error> {
        self.api_post("cancel_order/", request).await
    }

    /// Cancels all open orders. Calls POST /cancel_all_orders/.
    pub async fn cancel_all_orders(&self) -> Result<CancelAllOrdersResponse, Error> {
        self.api_post("cancel_all_orders/", Empty {}).await
    }

    /// Cancels all open orders for a market. Calls POST /cancel_all_orders/{market}/.
    pub async fn cancel_all_orders_for_market(
        &self,
        market: &str,
    ) -> Result<CancelAllOrdersResponse, Error> {
        let rest_method = format!("cancel_all_orders/{}/", market);
        self.api_post(rest_method.as_str(), Empty {}).await
    }

    /// Returns order status. Calls POST /order_status/.
    pub async fn get_order_status(
        &self,
        request: &OrderStatusRequest,
    ) -> Result<OrderStatusResponse, Error> {
        self.api_post("order_status/", request).await
    }

    /// Returns open orders for all markets. Calls POST /open_orders/.
    pub async fn get_open_orders(&self) -> Result<Vec<OpenOrder>, Error> {
        self.api_post("open_orders/", Empty {}).await
    }

    /// Returns open orders for a market. Calls POST /open_orders/{market}/.
    pub async fn get_open_orders_for_market(
        &self,
        market: &str,
    ) -> Result<Vec<OpenOrder>, Error> {
        let rest_method = format!("open_orders/{}/", market);
        self.api_post(rest_method.as_str(), Empty {}).await
    }

    /// Replaces an existing order. Calls POST /replace_order/.
    pub async fn replace_order(
        &self,
        request: &ReplaceOrderRequest,
    ) -> Result<ReplaceOrderResponse, Error> {
        self.api_post("replace_order/", request).await
    }

    /// Returns max order amount for a request. Calls POST /get_max_order_amount/.
    pub async fn get_max_order_amount(
        &self,
        request: &MaxOrderAmountRequest,
    ) -> Result<MaxOrderAmount, Error> {
        self.api_post("get_max_order_amount/", request).await
    }

    /// Returns the user's markets. Calls GET /my_markets/.
    pub async fn get_my_markets(&self) -> Result<Vec<TradingPair>, Error> {
        self.api_auth_get("my_markets/").await
    }

    /// Returns order events. Calls POST /order_data/.
    pub async fn get_order_data(
        &self,
        request: &OrderDataRequest,
    ) -> Result<Vec<OrderEvent>, Error> {
        self.api_post("order_data/", request).await
    }

    /// Returns account order events. Calls POST /account_order_data/.
    pub async fn get_account_order_data(
        &self,
        request: &AccountOrderDataRequest,
    ) -> Result<Vec<AccountOrderEvent>, Error> {
        self.api_post("account_order_data/", request).await
    }

    /// Returns estimated order impact. Calls POST /estimated_order_impact/.
    pub async fn get_estimated_order_impact(
        &self,
        request: &EstimatedOrderImpactRequest,
    ) -> Result<EstimatedOrderImpact, Error> {
        self.api_post("estimated_order_impact/", request).await
    }
}
