use crate::error::Error;
use crate::types::{TransferRequest, TransferResponse};

impl crate::client::Bitstamp {
    /// Transfers funds to the main account. Calls POST /transfer-to-main/.
    pub async fn transfer_to_main(&self, request: &TransferRequest) -> Result<TransferResponse, Error> {
        self.api_post("transfer-to-main/", request).await
    }

    /// Transfers funds from the main account. Calls POST /transfer-from-main/.
    pub async fn transfer_from_main(
        &self,
        request: &TransferRequest,
    ) -> Result<TransferResponse, Error> {
        self.api_post("transfer-from-main/", request).await
    }
}
