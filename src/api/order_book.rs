use crate::error::Error;
use crate::types::OrderBook;

impl crate::client::Bitstamp {
    /// Returns order book data for a market. Calls GET /order_book/{pair}/.
    pub async fn get_order_book(&self, currency_pair: &str, group: Option<&str>) -> Result<OrderBook, Error> {
        let rest_method = format!(
            "order_book/{}/{}",
            currency_pair,
            match group {
                None => "".to_string(),
                Some(g) => format!("?group={}", g),
            }
        );
        self.api_get(rest_method.as_str()).await
    }
}
