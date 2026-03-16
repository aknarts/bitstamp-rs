use crate::error::Error;
use crate::types::WebsocketToken;

impl crate::client::Bitstamp {
    /// Returns a WebSocket auth token. Calls POST /websockets_token/.
    pub async fn get_websocket_token(&self) -> Result<WebsocketToken, Error> {
        self.api_post("websockets_token/", crate::types::Empty {}).await
    }
}
