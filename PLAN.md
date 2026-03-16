# PLAN.md — bitstamp-rs Implementation Plan

## Overview

Full implementation of the Bitstamp API v2 client based on `openapi.json` (~85 endpoints).
Refactoring from a monolithic `lib.rs` into a modular `api/` + `types/` structure first.

## Phase 1: Refactor (Module Split)

Split the existing monolithic code into the target module structure without changing functionality.

| Step | Description | Status |
|------|-------------|--------|
| 1.1 | Split `types.rs` → `types/mod.rs`, `types/rest.rs`, `types/ws.rs`, `types/errors.rs` | ✅ |
| 1.2 | Extract `Bitstamp` struct + HTTP internals into `client.rs` | ✅ |
| 1.3 | Extract `BitstampEventStream` into `ws.rs` | ✅ |
| 1.4 | Move existing endpoint methods into `api/tickers.rs`, `api/market_info.rs`, `api/order_book.rs`, `api/transactions.rs`, `api/account.rs` | ✅ |
| 1.5 | Create `api/mod.rs` with re-exports | ✅ |
| 1.6 | Slim down `lib.rs` to module declarations + re-exports | ✅ |
| 1.7 | Verify: `cargo check`, `cargo test`, example builds, zero warnings | ✅ |
| 1.8 | Update `ARCHITECTURE.md` and `CODE_STYLE.md` | ✅ |

## Phase 2: Public API Endpoints (no auth)

| Module | Endpoint | Method | Status |
|--------|----------|--------|--------|
| `tickers` | `GET /ticker/` — All tickers | `get_tickers()` | ⬜ |
| `tickers` | `GET /ticker/` — All tickers | `get_tickers()` | ✅ |
| `tickers` | `GET /ticker/{market}/` — Market ticker | `get_ticker()` | ✅ |
| `tickers` | `GET /ticker_hour/{market}/` — Hourly ticker | `get_hourly_ticker()` | ✅ |
| `tickers` | `GET /currencies/` — Currencies list | `get_currencies()` | ✅ |
| `market_info` | `GET /markets/` — Markets list | `get_markets()` | ✅ |
| `market_info` | `GET /ohlc/{market}/` — OHLC data | `get_ohlc()` | ✅ |
| `market_info` | `GET /eur_usd/` — EUR/USD rate | `get_eur_usd()` | ✅ |
| `market_info` | `GET /funding_rate/{market}/` — Funding rate | `get_funding_rate()` | ✅ |
| `market_info` | `GET /funding_rate_history/{market}/` — Funding rate history | `get_funding_rate_history()` | ✅ |
| `order_book` | `GET /order_book/{market}/` — Order book | `get_order_book()` | ✅ |
| `transactions` | `GET /transactions/{market}/` — Transactions | `get_transactions()` | ✅ |

## Phase 3: Private API Endpoints (auth required)

### Account & Fees

| Module | Endpoint | Method | Status |
|--------|----------|--------|--------|
| `account` | `POST /account_balances/` — All balances | `get_account_balances()` | ✅ |
| `account` | `POST /account_balances/{currency}/` — Balance for currency | `get_account_balance()` | ✅ |
| `fees` | `POST /fees/trading/` — Trading fees | `get_trading_fees()` | ✅ |
| `fees` | `POST /fees/trading/{market}/` — Trading fee for market | `get_trading_fee()` | ✅ |
| `fees` | `POST /fees/withdrawal/` — Withdrawal fees | `get_withdrawal_fees()` | ✅ |
| `fees` | `POST /fees/withdrawal/{currency}/` — Withdrawal fee | `get_withdrawal_fee()` | ✅ |

### Orders

| Module | Endpoint | Method | Status |
|--------|----------|--------|--------|
| `orders` | `POST /buy/{market}/` — Buy limit order | `buy_limit_order()` | ✅ |
| `orders` | `POST /buy/market/{market}/` — Buy market order | `buy_market_order()` | ✅ |
| `orders` | `POST /buy/instant/{market}/` — Buy instant order | `buy_instant_order()` | ✅ |
| `orders` | `POST /sell/{market}/` — Sell limit order | `sell_limit_order()` | ✅ |
| `orders` | `POST /sell/market/{market}/` — Sell market order | `sell_market_order()` | ✅ |
| `orders` | `POST /sell/instant/{market}/` — Sell instant order | `sell_instant_order()` | ✅ |
| `orders` | `POST /cancel_order/` — Cancel order | `cancel_order()` | ✅ |
| `orders` | `POST /cancel_all_orders/` — Cancel all orders | `cancel_all_orders()` | ✅ |
| `orders` | `POST /cancel_all_orders/{market}/` — Cancel all for market | `cancel_all_orders_for_market()` | ✅ |
| `orders` | `POST /order_status/` — Order status | `get_order_status()` | ✅ |
| `orders` | `POST /open_orders/` — Open orders | `get_open_orders()` | ✅ |
| `orders` | `POST /open_orders/{market}/` — Open orders for market | `get_open_orders_for_market()` | ✅ |
| `orders` | `POST /replace_order/` — Replace order | `replace_order()` | ✅ |
| `orders` | `POST /get_max_order_amount/` — Max order amount | `get_max_order_amount()` | ✅ |
| `orders` | `GET /my_markets/` — Trading markets | `get_my_markets()` | ✅ |
| `orders` | `POST /order_data/` — Public order gap recovery | `get_order_data()` | ✅ |
| `orders` | `POST /account_order_data/` — Account order gap recovery | `get_account_order_data()` | ✅ |
| `orders` | `POST /estimated_order_impact/` — Estimated impact | `get_estimated_order_impact()` | ✅ |

### Transactions (Private)

| Module | Endpoint | Method | Status |
|--------|----------|--------|--------|
| `transactions` | `POST /user_transactions/` — User transactions | `get_user_transactions()` | ✅ |
| `transactions` | `POST /user_transactions/{market}/` — User txns for market | `get_user_transactions_for_market()` | ✅ |
| `transactions` | `POST /crypto-transactions/` — Crypto transactions | `get_crypto_transactions()` | ✅ |
| `transactions` | `GET /crypto-transactions/deposits/` — Crypto deposits | `get_crypto_deposits()` | ✅ |
| `transactions` | `POST /crypto-transactions/deposits/{id}/` — Update deposit | `update_crypto_deposit()` | ✅ |

### Derivatives

| Module | Endpoint | Method | Status |
|--------|----------|--------|--------|
| `derivatives` | `GET /open_positions/` — Open positions | `get_open_positions()` | ✅ |
| `derivatives` | `GET /open_positions/{market}/` — Positions by market | `get_open_positions_for_market()` | ✅ |
| `derivatives` | `GET /position_status/{id}/` — Position status | `get_position_status()` | ✅ |
| `derivatives` | `GET /position_history/` — Position history | `get_position_history()` | ✅ |
| `derivatives` | `GET /position_history/{market}/` — History by market | `get_position_history_for_market()` | ✅ |
| `derivatives` | `GET /position_settlement_transactions/` — Settlements | `get_position_settlements()` | ✅ |
| `derivatives` | `GET /position_settlement_transactions/{id}/` — Settlement by txn | `get_position_settlement()` | ✅ |
| `derivatives` | `POST /close_position/` — Close position | `close_position()` | ✅ |
| `derivatives` | `POST /close_positions/` — Close positions | `close_positions()` | ✅ |
| `derivatives` | `POST /adjust_position_collateral/` — Adjust collateral | `adjust_position_collateral()` | ✅ |
| `derivatives` | `POST /collateral_change_impact/` — Change impact | `get_collateral_change_impact()` | ✅ |
| `derivatives` | `GET /collateral_currencies/` — Collateral currencies | `get_collateral_currencies()` | ✅ |
| `derivatives` | `GET /leverage_settings/` — Leverage settings | `get_leverage_settings()` | ✅ |
| `derivatives` | `POST /leverage_settings/` — Update leverage | `update_leverage_settings()` | ✅ |
| `derivatives` | `GET /margin_info/` — Margin info | `get_margin_info()` | ✅ |
| `derivatives` | `GET /margin_tiers/` — Margin tiers | `get_margin_tiers()` | ✅ |
| `derivatives` | `GET /trade_history/` — Derivatives trade history | `get_trade_history()` | ✅ |
| `derivatives` | `GET /trade_history/{market}/` — Trade history by market | `get_trade_history_for_market()` | ✅ |

### Withdrawals & Deposits

| Module | Endpoint | Method | Status |
|--------|----------|--------|--------|
| `withdrawals` | `POST /withdrawal-requests/` — Withdrawal requests | `get_withdrawal_requests()` | ✅ |
| `withdrawals` | `POST /withdrawal/open/` — Open bank withdrawal | `open_bank_withdrawal()` | ✅ |
| `withdrawals` | `POST /withdrawal/status/` — Fiat withdrawal status | `get_withdrawal_status()` | ✅ |
| `withdrawals` | `POST /withdrawal/cancel/` — Cancel withdrawal | `cancel_withdrawal()` | ✅ |
| `withdrawals` | `POST /{currency}_withdrawal/` — Crypto withdrawal | `crypto_withdrawal()` | ✅ |
| `withdrawals` | `POST /ripple_withdrawal/` — Ripple IOU withdrawal | `ripple_withdrawal()` | ✅ |
| `deposits` | `POST /{currency}_address/` — Crypto deposit address | `get_crypto_deposit_address()` | ✅ |
| `deposits` | `POST /btc_unconfirmed/` — Unconfirmed BTC deposits | `get_btc_unconfirmed()` | ✅ |
| `deposits` | `POST /ripple_address/` — Ripple deposit address | `get_ripple_address()` | ✅ |

### Earn

| Module | Endpoint | Method | Status |
|--------|----------|--------|--------|
| `earn` | `POST /earn/subscribe/` — Subscribe to earn | `earn_subscribe()` | ✅ |
| `earn` | `POST /earn/unsubscribe/` — Unsubscribe from earn | `earn_unsubscribe()` | ✅ |
| `earn` | `GET /earn/subscriptions/` — Get subscriptions | `get_earn_subscriptions()` | ✅ |
| `earn` | `POST /earn/subscriptions/setting/` — Manage settings | `manage_earn_subscription_setting()` | ✅ |
| `earn` | `GET /earn/transactions/` — Get earn transactions | `get_earn_transactions()` | ✅ |

### Sub-account & WebSocket

| Module | Endpoint | Method | Status |
|--------|----------|--------|--------|
| `sub_account` | `POST /transfer-to-main/` — Transfer to main | `transfer_to_main()` | ✅ |
| `sub_account` | `POST /transfer-from-main/` — Transfer from main | `transfer_from_main()` | ✅ |
| `websocket` | `POST /websockets_token/` — WebSocket token | `get_websocket_token()` | ✅ |

### Travel Rule & Other

| Module | Endpoint | Method | Status |
|--------|----------|--------|--------|
| `travel_rule` | `GET /travel_rule/vasps/` — VASP list (public) | `get_vasps()` | ✅ |
| `travel_rule` | `GET /travel_rule/contacts/` — Get contacts | `get_travel_rule_contacts()` | ✅ |
| `travel_rule` | `POST /travel_rule/contacts/` — Create contact | `create_travel_rule_contact()` | ✅ |
| `travel_rule` | `GET /travel_rule/contacts/{uuid}/` — Get contact | `get_travel_rule_contact()` | ✅ |
| `travel_rule` | `POST /travel_rule/addresses/` — Submit address info | `submit_travel_rule_address()` | ✅ |
| `travel_rule` | `GET /travel_rule/satoshi_test/` — Get satoshi tests | `get_satoshi_tests()` | ✅ |
| `travel_rule` | `POST /travel_rule/satoshi_test/` — Create satoshi test | `create_satoshi_test()` | ✅ |
| `instant_convert` | `POST /instant_convert_address/info/` — Convert info | `get_instant_convert_address()` | ✅ |
| `instant_convert` | `POST /instant_convert_address/new/` — New convert addr | `new_instant_convert_address()` | ✅ |
| `security` | `POST /revoke_all_api_keys/` — Revoke all API keys | `revoke_all_api_keys()` | ✅ |

## Phase 4: Polish

| Step | Description | Status |
|------|-------------|--------|
| 4.1 | ~~Remove deprecated `get_trading_pairs_info()`~~ (replaced by `get_markets()`) | ✅ |
| 4.2 | ~~Remove old `get_balance()`~~ (already replaced by `get_account_balances()`) | ✅ |
| 4.3 | Update `CurrencyPairs` enum to support all pairs dynamically | ✅ |
| 4.4 | Add doc comments on all public types and methods | ✅ |
| 4.5 | Add basic unit tests (at least for serialization/deserialization) | ✅ |
| 4.6 | Update example to demonstrate new endpoints | ✅ |

## Status Legend

- ✅ Done
- 🔄 In Progress
- ⬜ Not Started
