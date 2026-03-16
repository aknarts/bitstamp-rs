use std::env;
use bitstamp::Bitstamp;
use bitstamp::types::UserTransactionsRequest;

#[tokio::main]
async fn main() {
    env_logger::init();
    let secret = env::var("BITSTAMP_ACCESS_SECRET").expect("BITSTAMP_ACCESS_SECRET not set");
    let key = env::var("BITSTAMP_ACCESS_KEY").expect("BITSTAMP_ACCESS_KEY not set");
    let bts = Bitstamp::new(secret, key);

    println!("=== Account Balances ===\n");

    let balances = bts.get_account_balances().await.expect("failed to fetch balances");
    let non_zero: Vec<_> = balances.iter()
        .filter(|b| b.total != "0.00000000" && b.total != "0.00")
        .collect();
    if non_zero.is_empty() {
        println!("No non-zero balances");
    } else {
        for b in &non_zero {
            println!("  {}: total={} available={} reserved={}", b.currency, b.total, b.available, b.reserved);
        }
    }

    println!("\n=== Trading Fees ===\n");

    let fees = bts.get_trading_fees().await.expect("failed to fetch trading fees");
    println!("{} fee entries", fees.len());
    for f in fees.iter().take(3) {
        println!("  {} — maker={} taker={}", f.market, f.fees.maker, f.fees.taker);
    }

    println!("\n=== Withdrawal Fees ===\n");

    let wfees = bts.get_withdrawal_fees().await.expect("failed to fetch withdrawal fees");
    for f in wfees.iter().take(5) {
        println!("  {} ({}) — fee={}", f.currency, f.network, f.fee);
    }

    println!("\n=== Open Orders ===\n");

    let orders = bts.get_open_orders().await.expect("failed to fetch open orders");
    if orders.is_empty() {
        println!("No open orders");
    } else {
        for o in &orders {
            println!("  #{} {} {} @ {} ({})", o.id, o.order_type, o.amount, o.price, o.market);
        }
    }

    println!("\n=== Recent Transactions ===\n");

    let req = UserTransactionsRequest {
        limit: Some("5".to_string()),
        ..Default::default()
    };
    let txns = bts.get_user_transactions(&req).await.expect("failed to fetch user transactions");
    for t in &txns {
        println!("  #{} type={} fee={} @ {}", t.id, t.type_field, t.fee, t.datetime);
    }

    println!("\n=== WebSocket Token ===\n");

    let token = bts.get_websocket_token().await.expect("failed to fetch websocket token");
    println!("Token: {}... (valid {}s, user_id={})", &token.token[..8], token.valid_sec, token.user_id);

    println!("\n=== My Markets (authenticated GET) ===\n");

    let my_markets = bts.get_my_markets().await.expect("failed to fetch my markets");
    println!("{} trading markets", my_markets.len());
    for m in my_markets.iter().take(5) {
        println!("  {} ({})", m.name, m.url_symbol);
    }

    println!("\n=== Earn Subscriptions (authenticated GET) ===\n");

    match bts.get_earn_subscriptions().await {
        Ok(subs) => {
            if subs.is_empty() {
                println!("No earn subscriptions");
            } else {
                for s in &subs {
                    println!("  {} {} — amount={} yield={}",
                        s.currency.as_deref().unwrap_or("?"),
                        s.type_field.as_deref().unwrap_or("?"),
                        s.amount,
                        s.estimated_annual_yield,
                    );
                }
            }
        }
        Err(e) => eprintln!("get_earn_subscriptions: {}", e),
    }
}
