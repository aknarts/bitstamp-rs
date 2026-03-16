use bitstamp::Bitstamp;

#[tokio::main]
async fn main() {
    let bts = Bitstamp::new(String::new(), String::new());

    let ticker = bts.get_ticker("btcusd").await.expect("failed to fetch ticker");
    println!("BTC/USD: {} (bid={} ask={})", ticker.last, ticker.bid, ticker.ask);
}
