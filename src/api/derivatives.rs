use crate::error::Error;
use crate::types::{
    AdjustCollateralRequest, ClosePositionRequest, ClosePositionsRequest, ClosedPosition,
    ClosedPositionsResponse, CollateralChangeImpact, CollateralChangeImpactRequest,
    CollateralCurrency, LeverageSettings, LeverageSettingsRequest, MarginInfo, MarketMarginTiers,
    OpenPosition, PositionHistory, PositionSettlement, PositionStatus, Trade,
};
use serde_json::Value;

impl crate::client::Bitstamp {
    /// Returns all open positions. Calls GET /open_positions/.
    pub async fn get_open_positions(&self) -> Result<Vec<OpenPosition>, Error> {
        self.api_auth_get("open_positions/").await
    }

    /// Returns open positions for a market. Calls GET /open_positions/{market}/.
    pub async fn get_open_positions_for_market(&self, market: &str) -> Result<Vec<OpenPosition>, Error> {
        let rest_method = format!("open_positions/{}/", market);
        self.api_auth_get(rest_method.as_str()).await
    }

    /// Returns position status by id. Calls GET /position_status/{id}/.
    pub async fn get_position_status(&self, id: &str) -> Result<PositionStatus, Error> {
        let rest_method = format!("position_status/{}/", id);
        self.api_auth_get(rest_method.as_str()).await
    }

    /// Returns position history. Calls GET /position_history/.
    pub async fn get_position_history(&self) -> Result<Vec<PositionHistory>, Error> {
        self.api_auth_get("position_history/").await
    }

    /// Returns position history for a market. Calls GET /position_history/{market}/.
    pub async fn get_position_history_for_market(&self, market: &str) -> Result<Vec<PositionHistory>, Error> {
        let rest_method = format!("position_history/{}/", market);
        self.api_auth_get(rest_method.as_str()).await
    }

    /// Returns position settlement transactions. Calls GET /position_settlement_transactions/.
    pub async fn get_position_settlements(&self) -> Result<Vec<PositionSettlement>, Error> {
        self.api_auth_get("position_settlement_transactions/").await
    }

    /// Returns a position settlement transaction. Calls GET /position_settlement_transactions/{id}/.
    pub async fn get_position_settlement(&self, transaction_id: &str) -> Result<Vec<PositionSettlement>, Error> {
        let rest_method = format!("position_settlement_transactions/{}/", transaction_id);
        self.api_auth_get(rest_method.as_str()).await
    }

    /// Closes a position. Calls POST /close_position/.
    pub async fn close_position(&self, request: &ClosePositionRequest) -> Result<ClosedPosition, Error> {
        self.api_post("close_position/", request).await
    }

    /// Closes multiple positions. Calls POST /close_positions/.
    pub async fn close_positions(&self, request: &ClosePositionsRequest) -> Result<ClosedPositionsResponse, Error> {
        self.api_post("close_positions/", request).await
    }

    /// Adjusts position collateral. Calls POST /adjust_position_collateral/.
    pub async fn adjust_position_collateral(&self, request: &AdjustCollateralRequest) -> Result<Value, Error> {
        self.api_post("adjust_position_collateral/", request).await
    }

    /// Returns collateral change impact. Calls POST /collateral_change_impact/.
    pub async fn get_collateral_change_impact(
        &self,
        request: &CollateralChangeImpactRequest,
    ) -> Result<CollateralChangeImpact, Error> {
        self.api_post("collateral_change_impact/", request).await
    }

    /// Returns collateral currencies. Calls GET /collateral_currencies/.
    pub async fn get_collateral_currencies(&self) -> Result<Vec<CollateralCurrency>, Error> {
        self.api_auth_get("collateral_currencies/").await
    }

    /// Returns leverage settings. Calls GET /leverage_settings/.
    pub async fn get_leverage_settings(&self) -> Result<Vec<LeverageSettings>, Error> {
        self.api_auth_get("leverage_settings/").await
    }

    /// Updates leverage settings. Calls POST /leverage_settings/.
    pub async fn update_leverage_settings(&self, request: &LeverageSettingsRequest) -> Result<LeverageSettings, Error> {
        self.api_post("leverage_settings/", request).await
    }

    /// Returns margin info. Calls GET /margin_info/.
    pub async fn get_margin_info(&self) -> Result<MarginInfo, Error> {
        self.api_auth_get("margin_info/").await
    }

    /// Returns margin tiers. Calls GET /margin_tiers/.
    pub async fn get_margin_tiers(&self) -> Result<Vec<MarketMarginTiers>, Error> {
        self.api_auth_get("margin_tiers/").await
    }

    /// Returns trade history. Calls GET /trade_history/.
    pub async fn get_trade_history(&self) -> Result<Vec<Trade>, Error> {
        self.api_auth_get("trade_history/").await
    }

    /// Returns trade history for a market. Calls GET /trade_history/{market}/.
    pub async fn get_trade_history_for_market(&self, market: &str) -> Result<Vec<Trade>, Error> {
        let rest_method = format!("trade_history/{}/", market);
        self.api_auth_get(rest_method.as_str()).await
    }
}
