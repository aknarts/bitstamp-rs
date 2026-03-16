use bitstamp::Bitstamp;
use bitstamp::types::Time;

#[tokio::main]
async fn main() {
    env_logger::init();
    let bts = Bitstamp::new(String::new(), String::new());

    println!("=== Currencies & Markets ===\n");

    let currencies = bts.get_currencies().await.expect("failed to fetch currencies");
    println!("{} currencies available:", currencies.len());
    for c in currencies.iter().take(5) {
        println!("  {} ({}) — {} decimals", c.name, c.currency, c.decimals);
    }

    let markets = bts.get_markets().await.expect("failed to fetch markets");
    println!("\n{} markets available:", markets.len());
    for m in markets.iter().take(5) {
        println!("  {} ({}) — type={}", m.name, m.market_symbol, m.market_type);
    }

    let rate = bts.get_eur_usd().await.expect("failed to fetch EUR/USD");
    println!("\nEUR/USD conversion: buy={} sell={}", rate.buy, rate.sell);

    println!("\n=== Order Book ===\n");

    let book = bts.get_order_book("btcusd", None).await.expect("failed to fetch order book");
    println!("BTC/USD order book: {} bids, {} asks", book.bids.len(), book.asks.len());
    if let (Some(best_bid), Some(best_ask)) = (book.bids.first(), book.asks.first()) {
        println!("  Best bid: {} @ {}", best_bid[1], best_bid[0]);
        println!("  Best ask: {} @ {}", best_ask[1], best_ask[0]);
    }

    println!("\n=== Recent Transactions ===\n");

    let txns = bts.get_transactions("btcusd", Some(Time::Minute)).await.expect("failed to fetch transactions");
    println!("{} trades in the last minute:", txns.len());
    for t in txns.iter().take(5) {
        let side = if t.type_field == "0" { "buy" } else { "sell" };
        println!("  {} {} BTC @ ${}", side, t.amount, t.price);
    }

    println!("\n=== OHLC Candles ===\n");

    let ohlc = bts.get_ohlc("btcusd", 3600, 5, None, None, None).await.expect("failed to fetch OHLC");
    println!("{} hourly candles for {}:", ohlc.data.ohlc.len(), ohlc.data.pair);
    for c in &ohlc.data.ohlc {
        println!("  {} — O={} H={} L={} C={} V={}", c.timestamp, c.open, c.high, c.low, c.close, c.volume);
    }

    println!("\n=== Funding Rates (Perpetuals) ===\n");

    match bts.get_funding_rate("btcusd-perp").await {
        Ok(fr) => println!("{}: rate={} next_funding={}", fr.market, fr.funding_rate, fr.next_funding_time),
        Err(e) => eprintln!("Funding rate unavailable: {}", e),
    }

    match bts.get_funding_rate_history("btcusd-perp", Some(3), None, None).await {
        Ok(frh) => {
            for tick in &frh.funding_rate_history {
                println!("  ts={} rate={}", tick.timestamp, tick.funding_rate);
            }
        }
        Err(e) => eprintln!("Funding rate history unavailable: {}", e),
    }
}
