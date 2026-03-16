use bitstamp::Bitstamp;
use bitstamp::types::{EventChannel, CurrencyPair};

#[tokio::main]
async fn main() {
    env_logger::init();
    let bts = Bitstamp::new(String::new(), String::new());

    let mut ws = bts.event_stream().await.expect("failed to connect WebSocket");

    ws.subscribe(EventChannel::LiveTrades(CurrencyPair::new("btcusd"))).await.expect("subscribe failed");
    println!("Subscribed to BTC/USD live trades. Waiting for events...\n");

    loop {
        match ws.next().await {
            Ok(event) => println!("{:?}: {:?}", event.event, event.data),
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
        }
    }
}
