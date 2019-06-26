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
pub struct TransactionStatus {
    /// If confirmed, the sequence of the block in which the transaction was executed
    #[serde(rename = "block_seq")]
    pub block_seq: Option<i64>,
    #[serde(rename = "confirmed")]
    pub confirmed: Option<bool>,
    /// If confirmed, how many blocks deep in the chain it is. Will be at least 1 if confirmed
    #[serde(rename = "height")]
    pub height: Option<i64>,
    #[serde(rename = "unconfirmed")]
    pub unconfirmed: Option<bool>,
}

impl TransactionStatus {
    pub fn new() -> TransactionStatus {
        TransactionStatus {
            block_seq: None,
            confirmed: None,
            height: None,
            unconfirmed: None,
        }
    }
}
