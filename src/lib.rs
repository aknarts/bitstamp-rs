#[macro_use]
extern crate log;

mod error;
pub mod types;

use crate::error::{Error, status_code, text_error, text_error_with_inner, v2_error, v1_error};

use serde::{de::DeserializeOwned, Serialize};
use hyper::{body::HttpBody, client::HttpConnector, Body, Client, Request};
use hyper_tls::HttpsConnector;

extern crate hmac;
extern crate sha2;

use sha2::Sha256;
use hmac::{Hmac, Mac, NewMac};
use std::str;
use uuid::Uuid;
use crate::types::Time;
use std::collections::HashMap;
use std::str::FromStr;
use chrono::Timelike;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio_tls::TlsStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;
use futures_util::{SinkExt, StreamExt};

const REST_HOST_PREFIX: &str = "www.bitstamp.net/api/v2";

type HmacSha256 = Hmac<Sha256>;
pub type WStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
type WebClient = Client<HttpsConnector<HttpConnector>, Body>;

pub struct Bitstamp {
    client: WebClient,
    secret: String,
    key: String,
}

pub struct BitstampEventStream {
    ws_stream: WStream,
    timeout: Duration,
}

impl BitstampEventStream {
    pub async fn next(&mut self) -> Result<types::Event, String> {
        loop {
            let next = self.ws_stream.next();
            match tokio::time::timeout(self.timeout, next).await {
                // Timed out
                Err(_) => return Err(format!("no activity for at least {:?}", self.timeout).to_string()),
                // Didn't time out
                Ok(next_result) => match next_result {
                    Some(msg) => match msg {
                        Ok(msg) => {
                            match self.handle_message(msg).await {
                                Ok(maybe_msg) => {
                                    if let Some(msg) = maybe_msg {
                                        return Ok(msg);
                                    } else {
                                        // Ignore other messages (but they'll reset the timeout)
                                        continue;
                                    }
                                }
                                Err(e) => return Err(e),
                            };
                        }
                        Err(e) => return Err(e.to_string()),
                    },
                    None => continue,
                },
            }
        }
    }

    async fn handle_message(&self, msg: Message) -> Result<Option<types::Event>, String> {
        match msg {
            Message::Binary(bytes) => match String::from_utf8(bytes) {
                Ok(json) => {
                    let json = json.as_str();
                    let event: Result<types::Event, _> = serde_json::from_str(json);
                    match event {
                        Ok(event) => Ok(Some(event)),
                        Err(e) => {
                            warn!("Couldn't deserialize: {:?}.  Original JSON:\n{}", e, &json);
                            Err(format!("unable to deserialize: {}", e))
                        }
                    }
                }
                Err(e) => Err(format!("UTF-8 decode failed: {}", e)),
            },
            Message::Text(t) => {
                let event: Result<types::Event, _> = serde_json::from_str(t.as_str());
                match event {
                    Ok(event) => Ok(Some(event)),
                    Err(e) => {
                        warn!("Couldn't deserialize: {:?}.  Original JSON:\n{}", e, &t);
                        Err(format!("unable to deserialize: {}", e))
                    }
                }
            }
            Message::Ping(_) => {
                debug!("Ping!");
                Ok(None)
            }
            Message::Close(t) => {
                debug!("close: {:?}", t);
                Ok(None)
            }
            Message::Pong(_) => {
                debug!("Pong!");
                Ok(None)
            }
        }
    }

    pub async fn subscribe(&mut self, channel: types::EventChannel) {
        self.ws_stream.send(Message::Text(serde_json::to_string(&types::OutEvent { event: types::EventEvent::BtsSubscribe, data: types::OutEventData { channel } }).unwrap())).await;
    }

    pub async fn unsubscribe(&mut self, channel: types::EventChannel) {
        self.ws_stream.send(Message::Text(serde_json::to_string(&types::OutEvent { event: types::EventEvent::BtsUnsubscribe, data: types::OutEventData { channel } }).unwrap())).await;
    }
}

impl Bitstamp {
    pub fn new(secret: String, key: String) -> Self {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        let mut bts = Bitstamp {
            client,
            secret,
            key,
        };
        bts
    }

    pub async fn event_stream(&self) -> Result<BitstampEventStream, Error> {
        let url = "wss://ws.bitstamp.net";
        match connect_async(url.clone()).await {
            Ok((mut ws_stream, _response)) => {
                debug!("Connected to {}", url);

                let timeout = Duration::from_secs(20);
                return Ok(BitstampEventStream { ws_stream, timeout });
            }
            Err(e) => {
                warn!("Failed to connect to {:?}: {:?}", url, e);
                return Err(text_error(format!("Failed to connect to {:?}: {:?}", url, e)));
            }
        };
    }

    /// Get a ticker
    pub async fn get_ticker(&self, currency_pair: &str) -> Result<types::Ticker, Error> {
        let rest_method = format!("ticker/{}/", currency_pair);
        self.api_get(rest_method.as_str()).await
    }

    /// Get a hourly ticker
    pub async fn get_hourly_ticker(&self, currency_pair: &str) -> Result<types::Ticker, Error> {
        let rest_method = format!("ticker_hour/{}/", currency_pair);
        self.api_get(rest_method.as_str()).await
    }

    /// Get an order book
    pub async fn get_order_book(&self, currency_pair: &str, group: Option<&str>) -> Result<types::OrderBook, Error> {
        let rest_method = format!("order_book/{}/{}", currency_pair, match group {
            None => { "".to_string() }
            Some(g) => { format!("?group={}", g) }
        });
        self.api_get(rest_method.as_str()).await
    }

    /// Get a transaction list
    pub async fn get_transactions(&self, currency_pair: &str, time: Option<Time>) -> Result<Vec<types::Transaction>, Error> {
        let rest_method = format!("transactions/{}/{}", currency_pair, match time {
            None => { "".to_string() }
            Some(t) => { format!("?time={}", t.to_string().to_lowercase()) }
        });
        self.api_get(rest_method.as_str()).await
    }

    /// Get a trading pair info
    pub async fn get_trading_pairs_info(&self) -> Result<Vec<types::PairInfo>, Error> {
        let rest_method = format!("trading-pairs-info/");
        self.api_get(rest_method.as_str()).await
    }

    /// Get EUR/USD conversion rate
    pub async fn get_eur_usd(&self) -> Result<types::ConversionRate, Error> {
        let rest_method = format!("eur_usd/");
        self.api_get(rest_method.as_str()).await
    }

    /// Get account ballance
    pub async fn get_balance(&self) -> Result<types::AccountBalance, Error> {
        let rest_method = format!("balance/");
        self.api_post(rest_method.as_str(), types::Offset { offset: "1".to_string() }).await
    }

    // PRIVATE

    async fn api_get<T: DeserializeOwned>(&self, rest_method: &str) -> Result<T, Error> {
        let body: Option<String> = None;
        self.rest_api("GET", rest_method, body).await
    }

    async fn api_post<T: DeserializeOwned, U: Serialize>(
        &self,
        rest_method: &str,
        body: U,
    ) -> Result<T, Error> {
        self.rest_api("POST", rest_method, Some(body)).await
    }

    async fn rest_api<T: DeserializeOwned, U: Serialize>(
        &self,
        http_method: &str,
        rest_method: &str,
        body: Option<U>,
    ) -> Result<T, Error> {
        match self.call_web_api_raw(http_method, rest_method, body).await {
            Ok(reply) => {
                let de: Result<T, _> = serde_json::from_str(reply.as_str());
                match de {
                    Ok(reply) => Ok(reply),
                    Err(e) => {
                        debug!("Couldn't parse reply for {} call: {}", rest_method, e);
                        debug!("Source JSON: {}", reply);
                        Err(text_error_with_inner(format!("failed to parse reply: {}", e), e))
                    }
                }
            }
            Err(e) => Err(e),
        }
    }

    async fn call_web_api_raw<T: Serialize>(
        &self,
        http_method: &str,
        rest_method: &str,
        body: Option<T>,
    ) -> Result<String, Error> {
        let prefix = String::from(REST_HOST_PREFIX);
        let url = format!("{}/{}", prefix, rest_method);


        debug!("Calling {} {:?}", http_method, url);
        let mut builder = Request::builder().method(http_method).uri(format!("https://{}", url));
        let body = if http_method.eq("POST") {
            let auth = format!("BITSTAMP {}", self.key);
            let nonce = Uuid::new_v4().to_string();
            let mut content_type = "application/x-www-form-urlencoded";
            let now = chrono::Utc::now();
            let timestamp = format!("{}{}", now.timestamp(), now.nanosecond() / 1000000); // TODO
            let mut payload = match body {
                Some(obj) => serde_json::to_string(&obj).unwrap(),
                None => "".to_string(),
            };
            let message = format!("{}POST{}{}{}{}v2{}", auth, url, content_type, nonce, timestamp, payload);
            debug!("{}", message);
            let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes()).expect("Failed to create hmac");
            mac.update(message.as_bytes());
            let mac_result = mac.finalize().into_bytes();
            let signature = hex::encode(mac_result);
            builder = builder.header("X-Auth", auth);
            builder = builder.header("X-Auth-Signature", signature);
            builder = builder.header("X-Auth-Nonce", nonce);
            builder = builder.header("X-Auth-Timestamp", timestamp);
            builder = builder.header("X-Auth-Version", "v2");
            builder = builder.header("Content-Type", content_type);
            Body::from(payload)
        } else {
            Body::empty()
        };
        debug!("{:?}", body);
        debug!("{:?}", builder.headers_ref().unwrap());

        let req = builder.body(body).unwrap();

        match self.client.request(req).await {
            Ok(mut resp) => {
                let mut reply = String::new();
                while let Some(chunk) = resp.body_mut().data().await {
                    use std::str;

                    let chunk = chunk.unwrap();
                    let strchunk = str::from_utf8(&chunk).unwrap();
                    reply.push_str(&strchunk);
                }
                if !resp.status().is_success() {
                    match serde_json::from_str::<types::V2Error>(&reply) {
                        Ok(status) => {
                            debug!("Request failed with {:#?}", status);
                            return Err(v2_error(resp.status(), status.reason, status.code));
                        }
                        Err(_) => {
                            match serde_json::from_str::<types::V1Error>(&reply) {
                                Ok(status) => {
                                    return Err(v1_error(resp.status(), status.error));
                                }
                                Err(_) => { return Err(status_code(resp.status())); }
                            }
                        }
                    };
                }
                Ok(reply)
            }
            Err(e) => Err(text_error_with_inner(format!("request failed: {}", e), e)),
        }
    }
}