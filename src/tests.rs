#[cfg(test)]
mod tests {
    use crate::types;
    use serde::de::DeserializeOwned;
    use serde::Serialize;
    use serde_json::Value;
    use std::str::FromStr;

    fn assert_round_trip<T>(value: &T)
    where
        T: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug,
    {
        let json = serde_json::to_string(value).unwrap();
        let decoded: T = serde_json::from_str(&json).unwrap();
        assert_eq!(*value, decoded);
    }

    #[test]
    fn rest_ticker_round_trip() {
        let json = r#"{
            "high":"12345.67",
            "last":"12000.01",
            "timestamp":"1680000000",
            "bid":"11995.00",
            "vwap":"12100.00",
            "volume":"42.0001",
            "low":"11800.00",
            "ask":"12005.00",
            "open":"11900.00"
        }"#;
        let ticker: types::Ticker = serde_json::from_str(json).unwrap();
        assert_eq!(ticker.high.as_str(), "12345.67");
        assert_eq!(ticker.volume.as_str(), "42.0001");
        assert_round_trip(&ticker);
    }

    #[test]
    fn rest_order_book_round_trip() {
        let json = r#"{
            "timestamp":"1680000000",
            "microtimestamp":"1680000000123456",
            "bids":[["12000.00","1.25"],["11950.00","0.5"]],
            "asks":[["12010.00","0.8"]]
        }"#;
        let book: types::OrderBook = serde_json::from_str(json).unwrap();
        assert_eq!(book.timestamp.as_str(), "1680000000");
        assert_eq!(book.bids[0][0].as_str(), "12000.00");
        assert_round_trip(&book);
    }

    #[test]
    fn rest_account_balance_round_trip() {
        let json = r#"{
            "currency":"USD",
            "total":"1000.00",
            "available":"800.00",
            "reserved":"200.00"
        }"#;
        let balance: types::AccountBalance = serde_json::from_str(json).unwrap();
        assert_eq!(balance.currency.as_str(), "USD");
        assert_eq!(balance.available.as_str(), "800.00");
        assert_round_trip(&balance);
    }

    #[test]
    fn rest_market_round_trip() {
        let json = r#"{
            "name":"BTC/USD",
            "market_symbol":"btcusd",
            "base_currency":"BTC",
            "base_decimals":8,
            "counter_currency":"USD",
            "counter_decimals":2,
            "minimum_order_value":"10.00",
            "maximum_order_value":"100000.00",
            "minimum_order_amount":"0.0001",
            "maximum_order_amount":"100.0",
            "trading":"Enabled",
            "instant_order_counter_decimals":2,
            "instant_and_market_orders":"True",
            "description":"Bitcoin / US Dollar",
            "market_type":"spot",
            "underlying_asset":"BTC",
            "payoff_type":"linear",
            "contract_size":"1",
            "tick_size":"0.01",
            "isin":"US1234567890"
        }"#;
        let market: types::Market = serde_json::from_str(json).unwrap();
        assert_eq!(market.market_symbol.as_str(), "btcusd");
        assert_eq!(market.base_decimals, 8);
        assert_eq!(market.minimum_order_value.as_str(), "10.00");
        assert_round_trip(&market);
    }

    #[test]
    fn rest_ohlc_response_round_trip() {
        let json = r#"{
            "data":{
                "pair":"btcusd",
                "market":"btcusd",
                "ohlc":[{
                    "timestamp":"1680000000",
                    "open":"12000.00",
                    "high":"12500.00",
                    "low":"11800.00",
                    "close":"12300.00",
                    "volume":"12.345"
                }]
            }
        }"#;
        let response: types::OhlcResponseWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.pair.as_str(), "btcusd");
        assert_eq!(response.data.ohlc.len(), 1);
        assert_eq!(response.data.ohlc[0].close.as_str(), "12300.00");
        assert_round_trip(&response);
    }

    #[test]
    fn rest_funding_rate_round_trip() {
        let json = r#"{
            "funding_rate":"0.0001",
            "timestamp":"1680000000",
            "market":"btcusd",
            "next_funding_time":"1680003600"
        }"#;
        let rate: types::FundingRate = serde_json::from_str(json).unwrap();
        assert_eq!(rate.funding_rate.as_str(), "0.0001");
        assert_eq!(rate.market.as_str(), "btcusd");
        assert_round_trip(&rate);
    }

    #[test]
    fn ws_currency_pair_round_trip() {
        let json = r#""BTCUSD""#;
        let pair: types::CurrencyPair = serde_json::from_str(json).unwrap();
        assert_eq!(pair.as_str(), "btcusd");
        assert_eq!(pair.to_string(), "btcusd");
        assert_round_trip(&pair);

        let from_str = types::CurrencyPair::from_str("ethusd").unwrap();
        assert_eq!(from_str.to_string(), "ethusd");
    }

    #[test]
    fn ws_event_channel_round_trip() {
        let json = r#""live_trades_btcusd""#;
        let channel: types::EventChannel = serde_json::from_str(json).unwrap();
        match &channel {
            types::EventChannel::LiveTrades(pair) => {
                assert_eq!(pair.as_str(), "btcusd");
            }
            _ => panic!("expected live trades channel"),
        }

        let encoded = serde_json::to_string(&channel).unwrap();
        assert_eq!(encoded, "\"live_trades_btcusd\"");
        let decoded: types::EventChannel = serde_json::from_str(&encoded).unwrap();
        match decoded {
            types::EventChannel::LiveTrades(pair) => {
                assert_eq!(pair.as_str(), "btcusd");
            }
            _ => panic!("expected live trades channel"),
        }
    }

    #[test]
    fn errors_v2_round_trip() {
        let json = r#"{
            "status":"error",
            "reason":"Invalid signature",
            "code":"200"
        }"#;
        let err: types::V2Error = serde_json::from_str(json).unwrap();
        assert_eq!(err.status.as_str(), "error");
        assert_eq!(err.code.as_str(), "200");
        assert_round_trip(&err);
    }

    #[test]
    fn errors_v1_round_trip() {
        let json = r#"{"error":"Missing parameter"}"#;
        let err: types::V1Error = serde_json::from_str(json).unwrap();
        assert_eq!(err.error.as_str(), "Missing parameter");
        assert_round_trip(&err);
    }

    #[test]
    fn deposits_deposit_address_round_trip() {
        let json = r#"{
            "address":"bc1qexample",
            "memo_id":"memo123",
            "destination_tag":12345,
            "transfer_id":987
        }"#;
        let address: types::DepositAddress = serde_json::from_str(json).unwrap();
        assert_eq!(address.address.as_str(), "bc1qexample");
        assert_eq!(address.memo_id.as_deref(), Some("memo123"));
        assert_eq!(address.destination_tag, Some(12345));
        assert_round_trip(&address);
    }

    #[test]
    fn derivatives_open_position_round_trip() {
        let json = r#"{
            "id":"pos-1",
            "market":"BTC/USD",
            "market_type":"spot",
            "margin_mode":"cross",
            "settlement_currency":"USD",
            "entry_price":"12000.00",
            "pnl_percentage":"0.05",
            "pnl_realized":"1.00",
            "pnl_settled_since_inception":"0.00",
            "leverage":"2",
            "pnl":"1.50",
            "cumulative_price_pnl":"1.23",
            "size":"0.10",
            "pnl_unrealized":"0.50",
            "pnl_in_settlement":"0.50",
            "implied_leverage":"1.8",
            "initial_margin":"600.00",
            "current_margin":"620.00",
            "collateral_reserved":"100.00",
            "maintenance_margin_ratio":"0.05",
            "estimated_liquidation_price":"9000.00",
            "estimated_closing_fee_amount":"0.10",
            "mark_price":"12100.00",
            "current_value":"1210.00",
            "entry_value":"1200.00",
            "strike_price":"0",
            "side":"long"
        }"#;
        let position: types::OpenPosition = serde_json::from_str(json).unwrap();
        assert_eq!(position.id.as_str(), "pos-1");
        assert_eq!(position.market.as_str(), "BTC/USD");
        assert_eq!(position.cumulative_price_pnl.as_deref(), Some("1.23"));
        assert_round_trip(&position);
    }

    #[test]
    fn derivatives_leverage_settings_round_trip() {
        let json = r#"{
            "margin_mode":"cross",
            "market":"BTC/USD",
            "leverage_current":"2",
            "leverage_max":"5"
        }"#;
        let settings: types::LeverageSettings = serde_json::from_str(json).unwrap();
        assert_eq!(settings.margin_mode.as_str(), "cross");
        assert_eq!(settings.leverage_max.as_str(), "5");
        assert_round_trip(&settings);
    }

    #[test]
    fn earn_subscription_round_trip() {
        let json = r#"{
            "currency":"BTC",
            "type":"fixed",
            "term":"30d",
            "estimated_annual_yield":0.05,
            "distribution_period":"monthly",
            "activation_period":"instant",
            "minimum_subscription_amount":0.001,
            "amount":0.01,
            "available_amount":0.02,
            "amount_earned":0.0001
        }"#;
        let subscription: types::EarnSubscription = serde_json::from_str(json).unwrap();
        assert_eq!(subscription.type_field.as_deref(), Some("fixed"));
        assert!((subscription.amount - 0.01).abs() < f64::EPSILON);
        assert_round_trip(&subscription);
    }

    #[test]
    fn fees_trading_fee_round_trip() {
        let json = r#"{
            "currency_pair":"btcusd",
            "market":"btcusd",
            "fees":{"maker":"0.001","taker":"0.002"}
        }"#;
        let fee: types::TradingFee = serde_json::from_str(json).unwrap();
        assert_eq!(fee.currency_pair.as_str(), "btcusd");
        assert_eq!(fee.fees.maker.as_str(), "0.001");
        assert_round_trip(&fee);
    }

    #[test]
    fn fees_withdrawal_fee_round_trip() {
        let json = r#"{
            "currency":"BTC",
            "fee":"0.0005",
            "network":"bitcoin"
        }"#;
        let fee: types::WithdrawalFee = serde_json::from_str(json).unwrap();
        assert_eq!(fee.currency.as_str(), "BTC");
        assert_eq!(fee.fee.as_str(), "0.0005");
        assert_round_trip(&fee);
    }

    #[test]
    fn orders_buy_sell_order_round_trip() {
        let json = r#"{
            "id":"12345",
            "market":"btcusd",
            "datetime":"2023-01-01 00:00:00",
            "type":"buy",
            "price":"12000.00",
            "amount":"0.5",
            "client_order_id":"client-1",
            "subtype":"limit",
            "margin_mode":"cross",
            "leverage":"2",
            "stop_price":"0",
            "trigger":"price",
            "reduce_only":false
        }"#;
        let order: types::BuySellOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(order.id.as_str(), "12345");
        assert_eq!(order.order_type.as_str(), "buy");
        assert!(!order.reduce_only);
        assert_round_trip(&order);
    }

    #[test]
    fn orders_open_order_round_trip() {
        let json = r#"{
            "id":"54321",
            "datetime":"2023-01-01 00:01:00",
            "type":"sell",
            "subtype":"limit",
            "price":"12500.00",
            "amount":"0.25",
            "amount_at_create":"0.25",
            "currency_pair":"btcusd",
            "market":"btcusd",
            "client_order_id":"client-2"
        }"#;
        let order: types::OpenOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order.id.as_str(), "54321");
        assert_eq!(order.order_type.as_str(), "sell");
        assert_round_trip(&order);
    }

    #[test]
    fn orders_cancel_order_round_trip() {
        let json = r#"{
            "id":"111",
            "amount":"0.10",
            "price":"12000.00",
            "type":"buy",
            "market":"btcusd",
            "status":"Finished"
        }"#;
        let order: types::CancelOrderResponse = serde_json::from_str(json).unwrap();
        assert_eq!(order.id.as_str(), "111");
        assert_eq!(order.status.as_str(), "Finished");
        assert_round_trip(&order);
    }

    #[test]
    fn sub_account_transfer_round_trip() {
        let json = r#"{"amount":"100.00","currency":"USD","subAccount":42}"#;
        let transfer: types::TransferRequest = serde_json::from_str(json).unwrap();
        assert_eq!(transfer.amount.as_str(), "100.00");
        assert_eq!(transfer.sub_account, 42);
        assert_round_trip(&transfer);

        let response_json = r#"{"status":"ok","reason":""}"#;
        let response: types::TransferResponse = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.status.as_str(), "ok");
        assert_round_trip(&response);
    }

    #[test]
    fn transactions_user_transaction_round_trip() {
        let json = r#"{
            "id":9876,
            "datetime":"2023-01-01 00:00:00",
            "type":"trade",
            "fee":"0.10",
            "order_id":12345,
            "self_trade":false,
            "self_trade_order_id":67890,
            "amount":"0.5",
            "price":"12000.00",
            "currency":"BTC"
        }"#;
        let tx: types::UserTransaction = serde_json::from_str(json).unwrap();
        assert_eq!(tx.id, 9876);
        assert_eq!(tx.order_id, Some(12345));
        assert_eq!(
            tx.other.get("amount"),
            Some(&Value::String("0.5".to_string()))
        );
        assert_round_trip(&tx);
    }

    #[test]
    fn transactions_crypto_deposit_round_trip() {
        let json = r#"{
            "id":555,
            "network":"bitcoin",
            "currency":"BTC",
            "destination_address":"bc1abc",
            "txid":"tx123",
            "amount":"0.05",
            "datetime":"2023-01-02 00:00:00",
            "status":"completed",
            "pending_reason":"confirmations"
        }"#;
        let deposit: types::CryptoDeposit = serde_json::from_str(json).unwrap();
        assert_eq!(deposit.id, 555);
        assert_eq!(deposit.pending_reason.as_deref(), Some("confirmations"));
        assert_round_trip(&deposit);
    }

    #[test]
    fn websocket_token_round_trip() {
        let json = r#"{"token":"token123","valid_sec":3600,"user_id":999}"#;
        let token: types::WebsocketToken = serde_json::from_str(json).unwrap();
        assert_eq!(token.token.as_str(), "token123");
        assert_eq!(token.valid_sec, 3600);
        assert_round_trip(&token);
    }

    #[test]
    fn withdrawals_withdrawal_request_round_trip() {
        let json = r#"{
            "id":1001,
            "datetime":"2023-01-03 00:00:00",
            "type":0,
            "currency":"BTC",
            "network":"bitcoin",
            "amount":"0.01",
            "status":"finished",
            "txid":"tx789",
            "address":"bc1xyz",
            "transaction_id":"tr-1"
        }"#;
        let request: types::WithdrawalRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.id, 1001);
        assert_eq!(request.currency.as_str(), "BTC");
        assert_round_trip(&request);
    }

    #[test]
    fn withdrawals_crypto_withdrawal_request_round_trip() {
        let json = r#"{
            "network":"bitcoin",
            "amount":"0.01",
            "address":"bc1xyz",
            "memo_id":"memo-1",
            "destination_tag":"12345",
            "transfer_id":"987",
            "originator_info":{"name":"Alice"},
            "beneficiary_info":{"name":"Bob"},
            "beneficiary_thirdparty":true,
            "beneficiary_id":"beneficiary-1",
            "vasp_uuid":"vasp-uuid"
        }"#;
        let request: types::CryptoWithdrawalRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.network.as_deref(), Some("bitcoin"));
        assert_eq!(request.memo_id.as_deref(), Some("memo-1"));
        assert_eq!(request.beneficiary_thirdparty, Some(true));
        assert_eq!(
            request.originator_info.as_ref().unwrap()["name"],
            Value::String("Alice".to_string())
        );
        assert_round_trip(&request);
    }

    #[test]
    fn travel_rule_vasp_round_trip() {
        let json = r#"{"uuid":"vasp-1","name":"Test VASP"}"#;
        let vasp: types::Vasp = serde_json::from_str(json).unwrap();
        assert_eq!(vasp.uuid.as_str(), "vasp-1");
        assert_round_trip(&vasp);
    }

    #[test]
    fn travel_rule_vasps_response_round_trip() {
        let json = r#"{
            "data":[{"uuid":"vasp-1","name":"Test VASP"}],
            "pagination":{"page":1,"size":10,"count":1}
        }"#;
        let response: types::VaspsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.pagination.as_ref().unwrap().page, 1);
        assert_round_trip(&response);
    }

    #[test]
    fn travel_rule_contact_round_trip() {
        let json = r#"{
            "id":"contact-1",
            "description":"Primary contact",
            "retail_info":{
                "first_name":"Alice",
                "last_name":"Doe",
                "date_of_birth":"1990-01-01",
                "city":"New York",
                "country":"US"
            }
        }"#;
        let contact: types::Contact = serde_json::from_str(json).unwrap();
        let retail = contact.retail_info.as_ref().unwrap();
        assert_eq!(retail.first_name.as_str(), "Alice");
        assert_eq!(contact.description.as_str(), "Primary contact");
        assert_round_trip(&contact);
    }

    #[test]
    fn travel_rule_satoshi_test_data_round_trip() {
        let json = r#"{
            "network":"bitcoin",
            "currency":"BTC",
            "amount":0.0001,
            "user_address":"bc1user",
            "deposit_address":"bc1deposit",
            "status":"pending",
            "expires":1680000000
        }"#;
        let data: types::SatoshiTestData = serde_json::from_str(json).unwrap();
        assert!((data.amount - 0.0001).abs() < 1e-12);
        assert_eq!(data.status.as_str(), "pending");
        assert_round_trip(&data);
    }

    #[test]
    fn instant_convert_address_info_round_trip() {
        let json = r#"{
            "address":"bc1convert",
            "currency_pair":"btcusd",
            "transactions":[{
                "order_id":1,
                "count":1,
                "trades":[{"exchange_rate":"12000.00","btc_amount":"0.01","fees":"0.0001"}]
            }]
        }"#;
        let info: types::InstantConvertAddressInfoResponse = serde_json::from_str(json).unwrap();
        assert_eq!(info.address.as_str(), "bc1convert");
        assert_eq!(info.transactions.len(), 1);
        assert_round_trip(&info);
    }

    #[test]
    fn security_revoked_api_keys_round_trip() {
        let json = r#"{"revoked_api_keys":["key1","key2"]}"#;
        let response: types::RevokedApiKeysResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.revoked_api_keys.len(), 2);
        assert_eq!(response.revoked_api_keys[0].as_str(), "key1");
        assert_round_trip(&response);
    }
}
