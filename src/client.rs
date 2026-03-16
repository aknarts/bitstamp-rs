use crate::error::{
    status_code, text_error, text_error_with_inner, v1_error, v2_error, Error,
};
use log::{debug, warn};
use serde::{de::DeserializeOwned, Serialize};
use hyper::{client::HttpConnector, Body, Client, Request};
use hyper_tls::HttpsConnector;
use sha2::Sha256;
use hmac::{Hmac, Mac, NewMac};
use std::str;
use uuid::Uuid;

use std::time::Duration;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

const REST_HOST_PREFIX: &str = "www.bitstamp.net/api/v2";

type HmacSha256 = Hmac<Sha256>;
type WebClient = Client<HttpsConnector<HttpConnector>, Body>;

/// Bitstamp REST and WebSocket client.
pub struct Bitstamp {
    pub(crate) client: WebClient,
    pub(crate) secret: String,
    pub(crate) key: String,
}

impl Bitstamp {
    /// Creates a new Bitstamp client with API credentials.
    pub fn new(secret: String, key: String) -> Self {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        let bts = Bitstamp {
            client,
            secret,
            key,
        };
        bts
    }

    /// Opens a WebSocket event stream connection.
    pub async fn event_stream(&self) -> Result<crate::ws::BitstampEventStream, Error> {
        let url = "wss://ws.bitstamp.net";
        let mut request = url
            .into_client_request()
            .map_err(|e| text_error_with_inner(format!("failed to build WebSocket request: {}", e), e))?;
        request.headers_mut().insert(
            "Origin",
            "https://www.bitstamp.net"
                .parse()
                .map_err(|e| text_error(format!("invalid Origin header value: {}", e)))?,
        );
        match connect_async(request).await {
            Ok((ws_stream, _response)) => {
                debug!("Connected to {}", url);
                let timeout = Duration::from_secs(20);
                Ok(crate::ws::BitstampEventStream { ws_stream, timeout })
            }
            Err(e) => {
                warn!("Failed to connect to {:?}: {:?}", url, e);
                Err(text_error(format!("Failed to connect to {:?}: {:?}", url, e)))
            }
        }
    }

    pub(crate) async fn api_get<T: DeserializeOwned>(&self, rest_method: &str) -> Result<T, Error> {
        let body: Option<String> = None;
        self.rest_api("GET", rest_method, body, false).await
    }

    pub(crate) async fn api_auth_get<T: DeserializeOwned>(&self, rest_method: &str) -> Result<T, Error> {
        let body: Option<String> = None;
        self.rest_api("GET", rest_method, body, true).await
    }

    pub(crate) async fn api_post<T: DeserializeOwned, U: Serialize>(
        &self,
        rest_method: &str,
        body: U,
    ) -> Result<T, Error> {
        self.rest_api("POST", rest_method, Some(body), true).await
    }

    async fn rest_api<T: DeserializeOwned, U: Serialize>(
        &self,
        http_method: &str,
        rest_method: &str,
        body: Option<U>,
        auth: bool,
    ) -> Result<T, Error> {
        match self.call_web_api_raw(http_method, rest_method, body, auth).await {
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
        auth: bool,
    ) -> Result<String, Error> {
        let prefix = String::from(REST_HOST_PREFIX);
        let url = format!("{}/{}", prefix, rest_method);

        debug!("Calling {} {:?}", http_method, url);
        let mut builder = Request::builder().method(http_method).uri(format!("https://{}", url));
        let body = if auth {
            let auth_header = format!("BITSTAMP {}", self.key);
            let nonce = Uuid::new_v4().to_string();
            let now = chrono::Utc::now();
            let timestamp = format!("{}", now.timestamp_millis());
            let payload = match body {
                Some(obj) => serde_json::to_string(&obj)
                    .map_err(|e| text_error_with_inner(format!("failed to serialize request body: {}", e), e))?,
                None => "".to_string(),
            };
            let has_body = !payload.is_empty();
            let content_type = if has_body { "application/x-www-form-urlencoded" } else { "" };
            let message = format!(
                "{}{}{}{}{}{}v2{}",
                auth_header, http_method, url, content_type, nonce, timestamp, payload
            );
            debug!("{}", message);
            let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes())
                .map_err(|e| text_error(format!("failed to create HMAC: {}", e)))?;
            mac.update(message.as_bytes());
            let mac_result = mac.finalize().into_bytes();
            let signature = hex::encode(mac_result);
            builder = builder.header("X-Auth", auth_header);
            builder = builder.header("X-Auth-Signature", signature);
            builder = builder.header("X-Auth-Nonce", nonce);
            builder = builder.header("X-Auth-Timestamp", timestamp);
            builder = builder.header("X-Auth-Version", "v2");
            if has_body {
                builder = builder.header("Content-Type", content_type);
            }
            Body::from(payload)
        } else {
            Body::empty()
        };
        debug!("{:?}", body);

        let req = builder
            .body(body)
            .map_err(|e| text_error_with_inner(format!("failed to build request: {}", e), e))?;

        match self.client.request(req).await {
            Ok(resp) => {
                let status = resp.status();
                let body_bytes = hyper::body::to_bytes(resp.into_body()).await
                    .map_err(|e| text_error_with_inner(format!("failed to read response body: {}", e), e))?;
                let reply = str::from_utf8(&body_bytes)
                    .map_err(|e| text_error_with_inner(format!("response body is not valid UTF-8: {}", e), e))?
                    .to_string();

                if !status.is_success() {
                    match serde_json::from_str::<crate::types::V2Error>(&reply) {
                        Ok(v2err) => {
                            debug!("Request failed with {:#?}", v2err);
                            return Err(v2_error(status, v2err.reason, v2err.code));
                        }
                        Err(_) => {
                            match serde_json::from_str::<crate::types::V1Error>(&reply) {
                                Ok(v1err) => {
                                    return Err(v1_error(status, v1err.error));
                                }
                                Err(_) => return Err(status_code(status)),
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
