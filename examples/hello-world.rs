use std::env;

extern crate env_logger;
extern crate bitstamp;

const BITSTAMP_ACCESS_SECRET: &'static str = "BITSTAMP_ACCESS_SECRET";
const BITSTAMP_ACCESS_KEY: &'static str = "BITSTAMP_ACCESS_KEY";

#[tokio::main]
async fn main() {
    env_logger::init();
    let secret = env::var(BITSTAMP_ACCESS_SECRET)
        .expect(format!("{} not specified in environment", BITSTAMP_ACCESS_SECRET).as_str());
    let key = env::var(BITSTAMP_ACCESS_KEY)
        .expect(format!("{} not specified in environment", BITSTAMP_ACCESS_KEY).as_str());


    let bts = bitstamp::Bitstamp::new(secret, key);

    match bts.get_ticker("btcusd").await {
        Ok(ticker) => { println!("{:?}", ticker); }
        Err(e) => { println!("{}", e); }
    }

    match bts.get_hourly_ticker("btcusd").await {
        Ok(ticker) => { println!("{:?}", ticker); }
        Err(e) => { println!("{}", e); }
    }

    match bts.get_order_book("btcusd", None).await {
        Ok(book) => { println!("{:?}", book); }
        Err(e) => { println!("{}", e); }
    }

    match bts.get_transactions("btcusd", Some(bitstamp::types::Time::Minute)).await {
        Ok(ledger) => { println!("{} {:?}", ledger.len(), ledger); }
        Err(e) => { println!("{}", e); }
    }

    match bts.get_trading_pairs_info().await {
        Ok(pairs) => { println!("{:?}", pairs); }
        Err(e) => { println!("{}", e); }
    }

    match bts.get_eur_usd().await {
        Ok(rate) => { println!("{:?}", rate); }
        Err(e) => { println!("{}", e); }
    }

    match bts.get_balance().await {
        Ok(rate) => { println!("{:#?}", rate); }
        Err(e) => { println!("{}", e); }
    }

    match bts.event_stream().await {
        Ok(mut ws) => {
            ws.subscribe(bitstamp::types::EventChannel::LiveTrades(bitstamp::types::CurrencyPairs::Btcusd)).await;
            ws.subscribe(bitstamp::types::EventChannel::LiveOrders(bitstamp::types::CurrencyPairs::Btcusd)).await;
            ws.subscribe(bitstamp::types::EventChannel::OrderBook(bitstamp::types::CurrencyPairs::Btcusd)).await;
            ws.subscribe(bitstamp::types::EventChannel::DetailOrderBook(bitstamp::types::CurrencyPairs::Btcusd)).await;
            ws.subscribe(bitstamp::types::EventChannel::DiffOrderBook(bitstamp::types::CurrencyPairs::Btcusd)).await;
            loop {
                match ws.next().await {
                    Ok(event) => { println!("Got event: {:?} at {:?}", event.event, event.data); }
                    Err(e) => { println!("{}", e); }
                }
            }
        }
        Err(e) => { println!("{}", e); }
    };
}
