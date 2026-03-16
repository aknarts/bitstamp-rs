use crate::error::Error;
use crate::types::{
    InstantConvertAddressInfoRequest, InstantConvertAddressInfoResponse,
    NewInstantConvertAddressRequest, NewInstantConvertAddressResponse,
};

impl crate::client::Bitstamp {
    /// Returns instant convert address info. Calls POST /instant_convert_address/info/.
    pub async fn get_instant_convert_address(
        &self,
        request: &InstantConvertAddressInfoRequest,
    ) -> Result<Vec<InstantConvertAddressInfoResponse>, Error> {
        self.api_post("instant_convert_address/info/", request).await
    }

    /// Creates a new instant convert address. Calls POST /instant_convert_address/new/.
    pub async fn new_instant_convert_address(
        &self,
        request: &NewInstantConvertAddressRequest,
    ) -> Result<NewInstantConvertAddressResponse, Error> {
        self.api_post("instant_convert_address/new/", request).await
    }
}
