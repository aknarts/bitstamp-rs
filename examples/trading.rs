use std::env;
use bitstamp::Bitstamp;
use bitstamp::types::{LimitOrderRequest, CancelOrderRequest, OrderStatusRequest};

#[tokio::main]
async fn main() {
    env_logger::init();
    let secret = env::var("BITSTAMP_ACCESS_SECRET").expect("BITSTAMP_ACCESS_SECRET not set");
    let key = env::var("BITSTAMP_ACCESS_KEY").expect("BITSTAMP_ACCESS_KEY not set");
    let bts = Bitstamp::new(secret, key);

    let market = "btcusd";

    println!("=== Place Limit Buy Order ===\n");

    let order_req = LimitOrderRequest {
        amount: Some("0.001".to_string()),
        price: "10000.00".to_string(),
        ..Default::default()
    };
    let order = bts.buy_limit_order(market, &order_req).await.expect("failed to place order");
    println!("Order placed: id={} market={} price={} amount={}", order.id, order.market, order.price, order.amount);

    println!("\n=== Check Order Status ===\n");

    let status_req = OrderStatusRequest {
        id: Some(order.id.clone()),
        ..Default::default()
    };
    let status = bts.get_order_status(&status_req).await.expect("failed to check status");
    println!("Order #{}: status={} remaining={}", status.id, status.status, status.amount_remaining);

    println!("\n=== Cancel Order ===\n");

    let cancel_req = CancelOrderRequest {
        id: Some(order.id.clone()),
        ..Default::default()
    };
    let cancelled = bts.cancel_order(&cancel_req).await.expect("failed to cancel order");
    println!("Cancelled: id={} market={} status={}", cancelled.id, cancelled.market, cancelled.status);
}
