/*
 * Skycoin REST API.
 *
 * Skycoin is a next-generation cryptocurrency.
 *
 * The version of the OpenAPI document: 0.26.0
 * Contact: contact@skycoin.net
 * Generated by: https://openapi-generator.tech
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineResponse2006 {
    #[serde(rename = "transactions")]
    pub transactions: Option<Vec<::models::TransactionEncoded>>,
}

impl InlineResponse2006 {
    pub fn new() -> InlineResponse2006 {
        InlineResponse2006 {
            transactions: None,
        }
    }
}
