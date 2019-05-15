/*
 * Skycoin REST API.
 *
 * Skycoin is a next-generation cryptocurrency.
 *
 * OpenAPI spec version: 0.25.1
 * Contact: contact@skycoin.net
 * Generated by: https://openapi-generator.tech
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiV1WalletTransactionHoursSelection {
    #[serde(rename = "mode")]
    pub mode: Option<String>,
    #[serde(rename = "share_factor")]
    pub share_factor: Option<String>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
}

impl ApiV1WalletTransactionHoursSelection {
    pub fn new() -> ApiV1WalletTransactionHoursSelection {
        ApiV1WalletTransactionHoursSelection {
            mode: None,
            share_factor: None,
            _type: None,
        }
    }
}