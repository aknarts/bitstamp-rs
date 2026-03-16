use crate::error::{text_error, text_error_with_inner, Error};
use crate::types;
use futures_util::{SinkExt, StreamExt};
use log::{debug, warn};
use serde_json;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;

/// WebSocket stream type for Bitstamp events.
pub type WStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// WebSocket event stream client.
pub struct BitstampEventStream {
    pub(crate) ws_stream: WStream,
    pub(crate) timeout: Duration,
}

impl BitstampEventStream {
    /// Receives the next WebSocket event.
    pub async fn next(&mut self) -> Result<types::Event, Error> {
        loop {
            let next = self.ws_stream.next();
            match tokio::time::timeout(self.timeout, next).await {
                Err(_) => return Err(text_error(format!("no activity for at least {:?}", self.timeout))),
                Ok(next_result) => match next_result {
                    Some(msg) => match msg {
                        Ok(msg) => match self.handle_message(msg).await {
                            Ok(maybe_msg) => {
                                if let Some(msg) = maybe_msg {
                                    return Ok(msg);
                                } else {
                                    continue;
                                }
                            }
                            Err(e) => return Err(e),
                        },
                        Err(e) => return Err(text_error_with_inner(format!("WebSocket error: {}", e), e)),
                    },
                    None => continue,
                },
            }
        }
    }

    async fn handle_message(&self, msg: Message) -> Result<Option<types::Event>, Error> {
        match msg {
            Message::Binary(bytes) => match String::from_utf8(bytes) {
                Ok(json) => {
                    let json = json.as_str();
                    let event: Result<types::Event, _> = serde_json::from_str(json);
                    match event {
                        Ok(event) => Ok(Some(event)),
                        Err(e) => {
                            warn!("Couldn't deserialize: {:?}.  Original JSON:\n{}", e, &json);
                            Err(text_error_with_inner(format!("unable to deserialize: {}", e), e))
                        }
                    }
                }
                Err(e) => Err(text_error_with_inner(format!("UTF-8 decode failed: {}", e), e)),
            },
            Message::Text(t) => {
                let event: Result<types::Event, _> = serde_json::from_str(t.as_str());
                match event {
                    Ok(event) => Ok(Some(event)),
                    Err(e) => {
                        warn!("Couldn't deserialize: {:?}.  Original JSON:\n{}", e, &t);
                        Err(text_error_with_inner(format!("unable to deserialize: {}", e), e))
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

    /// Subscribes to a WebSocket channel.
    pub async fn subscribe(&mut self, channel: types::EventChannel) -> Result<(), Error> {
        let msg = serde_json::to_string(&types::OutEvent {
            event: types::EventEvent::BtsSubscribe,
            data: types::OutEventData { channel },
        })
        .map_err(|e| text_error_with_inner(format!("failed to serialize subscribe message: {}", e), e))?;
        self.ws_stream
            .send(Message::Text(msg))
            .await
            .map_err(|e| text_error_with_inner(format!("failed to send subscribe message: {}", e), e))
    }

    /// Unsubscribes from a WebSocket channel.
    pub async fn unsubscribe(&mut self, channel: types::EventChannel) -> Result<(), Error> {
        let msg = serde_json::to_string(&types::OutEvent {
            event: types::EventEvent::BtsUnsubscribe,
            data: types::OutEventData { channel },
        })
        .map_err(|e| text_error_with_inner(format!("failed to serialize unsubscribe message: {}", e), e))?;
        self.ws_stream
            .send(Message::Text(msg))
            .await
            .map_err(|e| text_error_with_inner(format!("failed to send unsubscribe message: {}", e), e))
    }
}
