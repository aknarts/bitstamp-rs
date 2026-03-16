use serde::{Deserialize, Serialize};

/// WebSocket token response returned by `POST /websockets_token/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebsocketToken {
    pub token: String,
    pub valid_sec: i64,
    pub user_id: i64,
}
