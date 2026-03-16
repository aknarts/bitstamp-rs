use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Open position data returned by `GET /open_positions/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenPosition {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub market: String,
    #[serde(default)]
    pub market_type: String,
    #[serde(default)]
    pub margin_mode: String,
    #[serde(default)]
    pub settlement_currency: String,
    #[serde(default)]
    pub entry_price: String,
    #[serde(default)]
    pub pnl_percentage: String,
    #[serde(default)]
    pub pnl_realized: String,
    #[serde(default)]
    pub pnl_settled_since_inception: String,
    #[serde(default)]
    pub leverage: String,
    #[serde(default)]
    pub pnl: String,
    #[serde(default)]
    pub cumulative_price_pnl: Option<String>,
    #[serde(default)]
    pub cumulative_trading_fees: Option<String>,
    #[serde(default)]
    pub cumulative_liquidation_fees: Option<String>,
    #[serde(default)]
    pub cumulative_funding: Option<String>,
    #[serde(default)]
    pub cumulative_socialized_loss: Option<String>,
    #[serde(default)]
    pub size: String,
    #[serde(default)]
    pub pnl_unrealized: String,
    #[serde(default)]
    pub pnl_in_settlement: String,
    #[serde(default)]
    pub implied_leverage: Option<String>,
    #[serde(default)]
    pub initial_margin: String,
    #[serde(default)]
    pub initial_margin_ratio: Option<String>,
    #[serde(default)]
    pub current_margin: String,
    #[serde(default)]
    pub collateral_reserved: String,
    #[serde(default)]
    pub maintenance_margin: Option<String>,
    #[serde(default)]
    pub maintenance_margin_ratio: String,
    #[serde(default)]
    pub estimated_liquidation_price: String,
    #[serde(default)]
    pub estimated_closing_fee_amount: String,
    #[serde(default)]
    pub mark_price: String,
    #[serde(default)]
    pub current_value: String,
    #[serde(default)]
    pub entry_value: String,
    #[serde(default)]
    pub strike_price: String,
    #[serde(default)]
    pub side: String,
    #[serde(default)]
    pub margin_tier: Option<String>,
}

/// Position status response returned by `GET /position_status/{id}/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PositionStatus {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub market: String,
    #[serde(default)]
    pub market_type: String,
    #[serde(default)]
    pub margin_mode: String,
    #[serde(default)]
    pub settlement_currency: String,
    #[serde(default)]
    pub entry_price: String,
    #[serde(default)]
    pub pnl_percentage: String,
    #[serde(default)]
    pub pnl_realized: String,
    #[serde(default)]
    pub pnl_settled_since_inception: String,
    #[serde(default)]
    pub leverage: String,
    #[serde(default)]
    pub pnl: String,
    #[serde(default)]
    pub cumulative_price_pnl: Option<String>,
    #[serde(default)]
    pub cumulative_trading_fees: Option<String>,
    #[serde(default)]
    pub cumulative_liquidation_fees: Option<String>,
    #[serde(default)]
    pub cumulative_funding: Option<String>,
    #[serde(default)]
    pub cumulative_socialized_loss: Option<String>,
    #[serde(default)]
    pub pnl_in_settlement: String,
    #[serde(default)]
    pub time_opened: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub margin_tier: Option<String>,
    #[serde(default)]
    pub side: String,
    #[serde(default)]
    pub size: String,
    #[serde(default)]
    pub pnl_unrealized: String,
    #[serde(default)]
    pub implied_leverage: Option<String>,
    #[serde(default)]
    pub initial_margin: String,
    #[serde(default)]
    pub initial_margin_ratio: Option<String>,
    #[serde(default)]
    pub current_margin: String,
    #[serde(default)]
    pub collateral_reserved: String,
    #[serde(default)]
    pub maintenance_margin: Option<String>,
    #[serde(default)]
    pub maintenance_margin_ratio: String,
    #[serde(default)]
    pub estimated_liquidation_price: String,
    #[serde(default)]
    pub estimated_closing_fee_amount: String,
    #[serde(default)]
    pub mark_price: String,
    #[serde(default)]
    pub current_value: String,
    #[serde(default)]
    pub entry_value: String,
    #[serde(default)]
    pub strike_price: String,
    #[serde(default)]
    pub exit_price: String,
    #[serde(default)]
    pub settlement_price: Option<String>,
    #[serde(default)]
    pub amount_delta: String,
    #[serde(default)]
    pub time_closed: String,
}

/// Position history entry returned by `GET /position_history/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PositionHistory {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub market: String,
    #[serde(default)]
    pub market_type: String,
    #[serde(default)]
    pub margin_mode: String,
    #[serde(default)]
    pub pnl_currency: String,
    #[serde(default)]
    pub entry_price: String,
    #[serde(default)]
    pub pnl_percentage: String,
    #[serde(default)]
    pub pnl_realized: String,
    #[serde(default)]
    pub pnl_settled: String,
    #[serde(default)]
    pub leverage: String,
    #[serde(default)]
    pub pnl: String,
    #[serde(default)]
    pub cumulative_price_pnl: Option<String>,
    #[serde(default)]
    pub cumulative_trading_fees: Option<String>,
    #[serde(default)]
    pub cumulative_liquidation_fees: Option<String>,
    #[serde(default)]
    pub cumulative_funding: Option<String>,
    #[serde(default)]
    pub cumulative_socialized_loss: Option<String>,
    #[serde(default)]
    pub amount_delta: String,
    #[serde(default)]
    pub time_opened: String,
    #[serde(default)]
    pub time_closed: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub exit_price: String,
    #[serde(default)]
    pub settlement_price: String,
}

/// Position settlement transaction returned by `GET /position_settlement_transactions/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PositionSettlement {
    #[serde(default)]
    pub transaction_id: String,
    #[serde(default)]
    pub position_id: String,
    #[serde(default)]
    pub settlement_time: i64,
    #[serde(default)]
    pub settlement_type: String,
    #[serde(default)]
    pub settlement_price: Option<String>,
    #[serde(default)]
    pub market: String,
    #[serde(default)]
    pub market_type: String,
    #[serde(default)]
    pub pnl_currency: String,
    #[serde(default)]
    pub pnl_settled: String,
    #[serde(default)]
    pub pnl_component_price: String,
    #[serde(default)]
    pub pnl_component_fees: String,
    #[serde(default)]
    pub pnl_component_funding: String,
    #[serde(default)]
    pub pnl_component_socialized_loss: String,
    #[serde(default)]
    pub margin_mode: String,
    #[serde(default)]
    pub size: String,
    #[serde(default)]
    pub strike_price: String,
    #[serde(default)]
    pub fees_component_trading: String,
    #[serde(default)]
    pub fees_component_liquidation: String,
}

/// Closed position response from `POST /close_position/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClosedPosition {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub market: String,
    #[serde(default)]
    pub market_type: String,
    #[serde(default)]
    pub margin_mode: String,
    #[serde(default)]
    pub pnl_currency: String,
    #[serde(default)]
    pub entry_price: String,
    #[serde(default)]
    pub pnl_percentage: String,
    #[serde(default)]
    pub pnl_realized: String,
    #[serde(default)]
    pub pnl_settled: String,
    #[serde(default)]
    pub leverage: String,
    #[serde(default)]
    pub pnl: String,
    #[serde(default)]
    pub cumulative_price_pnl: Option<String>,
    #[serde(default)]
    pub cumulative_trading_fees: Option<String>,
    #[serde(default)]
    pub cumulative_liquidation_fees: Option<String>,
    #[serde(default)]
    pub cumulative_funding: Option<String>,
    #[serde(default)]
    pub cumulative_socialized_loss: Option<String>,
    #[serde(default)]
    pub amount_delta: String,
    #[serde(default)]
    pub time_opened: String,
    #[serde(default)]
    pub time_closed: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub exit_price: String,
    #[serde(default)]
    pub settlement_price: String,
    #[serde(default)]
    pub closing_fee_amount: String,
}

/// Closed positions response from `POST /close_positions/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClosedPositionsResponse {
    #[serde(default)]
    pub closed: Vec<ClosedPosition>,
    #[serde(default)]
    pub failed: Vec<ClosedPosition>,
}

/// Request to close a single position.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClosePositionRequest {
    pub position_id: String,
}

/// Request to close multiple positions.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClosePositionsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_mode: Option<String>,
    pub order_type: String,
}

/// Request to adjust collateral for a position.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdjustCollateralRequest {
    pub position_id: String,
    pub new_amount: String,
}

/// Request to estimate collateral change impact.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollateralChangeImpactRequest {
    pub margin_mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_collateral: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collateral_deltas: Option<Value>,
}

/// Request to update leverage settings.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeverageSettingsRequest {
    pub margin_mode: String,
    pub market: String,
    pub leverage: String,
}

/// Collateral change impact response from `POST /collateral_change_impact/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollateralChangeImpact {
    #[serde(default)]
    pub margin_currency: String,
    #[serde(default)]
    pub estimated_initial_margin_ratio: String,
    #[serde(default)]
    pub estimated_maintenance_margin_ratio: String,
    #[serde(default)]
    pub estimated_liquidation_prices: Value,
    #[serde(default)]
    pub total_estimated_margin: String,
}

/// Collateral currency entry returned by `GET /collateral_currencies/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollateralCurrency {
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub haircut: String,
}

/// Leverage settings entry returned by `GET /leverage_settings/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeverageSettings {
    #[serde(default)]
    pub margin_mode: String,
    #[serde(default)]
    pub market: String,
    #[serde(default)]
    pub leverage_current: String,
    #[serde(default)]
    pub leverage_max: String,
}

/// Margin info returned by `GET /margin_info/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarginInfo {
    #[serde(default)]
    pub account_margin: String,
    #[serde(default)]
    pub account_margin_available: String,
    #[serde(default)]
    pub account_margin_reserved: String,
    #[serde(default)]
    pub account_margin_currency: String,
    #[serde(default)]
    pub assets: Vec<Value>,
    #[serde(default)]
    pub initial_margin_ratio: Option<String>,
    #[serde(default)]
    pub maintenance_margin_ratio: Option<String>,
    #[serde(default)]
    pub implied_leverage: Option<String>,
}

/// Market margin tier data returned by `GET /margin_tiers/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketMarginTiers {
    #[serde(default)]
    pub market: String,
    #[serde(default)]
    pub tiers: Vec<Value>,
}

/// Trade history entry returned by `GET /trade_history/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trade {
    #[serde(default)]
    pub trade_id: String,
    #[serde(default)]
    pub order_id: String,
    #[serde(default)]
    pub self_trade_order_id: String,
    #[serde(default)]
    pub datetime: String,
    #[serde(default)]
    pub fee: String,
    #[serde(default)]
    pub liquidation_fee: String,
    #[serde(default)]
    pub fee_currency: String,
    #[serde(default)]
    pub market: String,
    #[serde(default)]
    pub margin_mode: String,
    #[serde(default)]
    pub leverage: String,
    #[serde(default)]
    pub side: String,
    #[serde(rename = "type", default)]
    pub trade_type: String,
    #[serde(default)]
    pub self_trade_type: String,
    #[serde(default)]
    pub price: String,
    #[serde(default)]
    pub amount: String,
}
