use crate::error::Error;
use crate::types::{
    Contact, CreateContactRequest, CreateSatoshiTestRequest, SatoshiTestData,
    TravelRuleAddress, VaspsResponse,
};

impl crate::client::Bitstamp {
    /// Returns VASP list. Calls GET /travel_rule/vasps/.
    pub async fn get_vasps(
        &self,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<VaspsResponse, Error> {
        let mut params = Vec::new();
        if let Some(p) = page {
            params.push(format!("page={}", p));
        }
        if let Some(pp) = per_page {
            params.push(format!("per_page={}", pp));
        }
        let query = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        let rest_method = format!("travel_rule/vasps/{}", query);
        self.api_get(rest_method.as_str()).await
    }

    /// Returns travel rule contacts. Calls GET /travel_rule/contacts/.
    pub async fn get_travel_rule_contacts(&self) -> Result<Vec<Contact>, Error> {
        self.api_auth_get("travel_rule/contacts/").await
    }

    /// Creates a travel rule contact. Calls POST /travel_rule/contacts/.
    pub async fn create_travel_rule_contact(
        &self,
        request: &CreateContactRequest,
    ) -> Result<Contact, Error> {
        self.api_post("travel_rule/contacts/", request).await
    }

    /// Returns a travel rule contact. Calls GET /travel_rule/contacts/{uuid}/.
    pub async fn get_travel_rule_contact(&self, contact_uuid: &str) -> Result<Contact, Error> {
        let rest_method = format!("travel_rule/contacts/{}/", contact_uuid);
        self.api_auth_get(rest_method.as_str()).await
    }

    /// Submits a travel rule address. Calls POST /travel_rule/addresses/.
    pub async fn submit_travel_rule_address(
        &self,
        request: &TravelRuleAddress,
    ) -> Result<TravelRuleAddress, Error> {
        self.api_post("travel_rule/addresses/", request).await
    }

    /// Returns satoshi tests. Calls GET /travel_rule/satoshi_test/.
    pub async fn get_satoshi_tests(&self) -> Result<Vec<SatoshiTestData>, Error> {
        self.api_auth_get("travel_rule/satoshi_test/").await
    }

    /// Creates a satoshi test. Calls POST /travel_rule/satoshi_test/.
    pub async fn create_satoshi_test(
        &self,
        request: &CreateSatoshiTestRequest,
    ) -> Result<SatoshiTestData, Error> {
        self.api_post("travel_rule/satoshi_test/", request).await
    }
}
