use crate::error::Error;
use crate::types::{
    CancelWithdrawalRequest, CancelWithdrawalResponse, CryptoWithdrawalRequest, CryptoWithdrawalResponse,
    OpenBankWithdrawalRequest, OpenBankWithdrawalResponse, RippleWithdrawalRequest, RippleWithdrawalResponse,
    WithdrawalRequest, WithdrawalRequestsRequest, BankWithdrawalStatusRequest, BankWithdrawalStatusResponse,
};

impl crate::client::Bitstamp {
    /// Returns withdrawal requests. Calls POST /withdrawal-requests/.
    pub async fn get_withdrawal_requests(
        &self,
        request: &WithdrawalRequestsRequest,
    ) -> Result<Vec<WithdrawalRequest>, Error> {
        self.api_post("withdrawal-requests/", request).await
    }

    /// Opens a bank withdrawal. Calls POST /withdrawal/open/.
    pub async fn open_bank_withdrawal(
        &self,
        request: &OpenBankWithdrawalRequest,
    ) -> Result<OpenBankWithdrawalResponse, Error> {
        self.api_post("withdrawal/open/", request).await
    }

    /// Returns bank withdrawal status. Calls POST /withdrawal/status/.
    pub async fn get_withdrawal_status(
        &self,
        request: &BankWithdrawalStatusRequest,
    ) -> Result<BankWithdrawalStatusResponse, Error> {
        self.api_post("withdrawal/status/", request).await
    }

    /// Cancels a withdrawal. Calls POST /withdrawal/cancel/.
    pub async fn cancel_withdrawal(
        &self,
        request: &CancelWithdrawalRequest,
    ) -> Result<CancelWithdrawalResponse, Error> {
        self.api_post("withdrawal/cancel/", request).await
    }

    /// Creates a crypto withdrawal. Calls POST /{currency}_withdrawal/.
    pub async fn crypto_withdrawal(
        &self,
        currency: &str,
        request: &CryptoWithdrawalRequest,
    ) -> Result<CryptoWithdrawalResponse, Error> {
        let rest_method = format!("{}_withdrawal/", currency);
        self.api_post(rest_method.as_str(), request).await
    }

    /// Creates a ripple withdrawal. Calls POST /ripple_withdrawal/.
    pub async fn ripple_withdrawal(
        &self,
        request: &RippleWithdrawalRequest,
    ) -> Result<RippleWithdrawalResponse, Error> {
        self.api_post("ripple_withdrawal/", request).await
    }
}
