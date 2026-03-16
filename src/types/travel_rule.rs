use serde::{Deserialize, Serialize};

/// VASP entry returned by `GET /travel_rule/vasps/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vasp {
    #[serde(default)]
    pub uuid: String,
    #[serde(default)]
    pub name: String,
}

/// Pagination info returned by travel rule list endpoints.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pagination {
    #[serde(default)]
    pub page: i64,
    #[serde(default)]
    pub size: i64,
    #[serde(default)]
    pub count: i64,
}

/// VASP list response from `GET /travel_rule/vasps/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VaspsResponse {
    #[serde(default)]
    pub data: Vec<Vasp>,
    #[serde(default)]
    pub pagination: Option<Pagination>,
}

/// Retail contact information for travel rule requests.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetailInfo {
    pub first_name: String,
    pub last_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_birth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_and_house_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

/// Corporate contact information for travel rule requests.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CorporateInfo {
    pub company_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lei: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
}

/// Travel rule contact entry returned by `GET /travel_rule/contacts/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contact {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retail_info: Option<RetailInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corporate_info: Option<CorporateInfo>,
}

/// Create contact request payload for `POST /travel_rule/contacts/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateContactRequest {
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retail_info: Option<RetailInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corporate_info: Option<CorporateInfo>,
}

/// Travel rule address request payload for `POST /travel_rule/addresses/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TravelRuleAddress {
    pub address: String,
    pub network: String,
    pub contact_thirdparty: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_uuid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vasp_uuid: Option<String>,
}

/// Satoshi test entry returned by `GET /travel_rule/satoshi_test/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SatoshiTestData {
    #[serde(default)]
    pub network: String,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub amount: f64,
    #[serde(default)]
    pub user_address: String,
    #[serde(default)]
    pub deposit_address: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub expires: i64,
}

/// Satoshi test request payload for `POST /travel_rule/satoshi_test/`.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateSatoshiTestRequest {
    pub address: String,
    pub network: String,
    pub currency: String,
}
