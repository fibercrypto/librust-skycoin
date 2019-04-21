/* 
 * Skycoin REST API.
 *
 * Skycoin is a next-generation cryptocurrency.
 *
 * OpenAPI spec version: 0.25.1
 * Contact: contact@skycoin.com
 * Generated by: https://openapi-generator.tech
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Apiv1exploreraddressOutputs {
  #[serde(rename = "hours")]
  hours: Option<i64>,
  #[serde(rename = "dst")]
  dst: Option<String>,
  #[serde(rename = "coins")]
  coins: Option<String>,
  #[serde(rename = "uxid")]
  uxid: Option<String>
}

impl Apiv1exploreraddressOutputs {
  pub fn new() -> Apiv1exploreraddressOutputs {
    Apiv1exploreraddressOutputs {
      hours: None,
      dst: None,
      coins: None,
      uxid: None
    }
  }

  pub fn set_hours(&mut self, hours: i64) {
    self.hours = Some(hours);
  }

  pub fn with_hours(mut self, hours: i64) -> Apiv1exploreraddressOutputs {
    self.hours = Some(hours);
    self
  }

  pub fn hours(&self) -> Option<&i64> {
    self.hours.as_ref()
  }

  pub fn reset_hours(&mut self) {
    self.hours = None;
  }

  pub fn set_dst(&mut self, dst: String) {
    self.dst = Some(dst);
  }

  pub fn with_dst(mut self, dst: String) -> Apiv1exploreraddressOutputs {
    self.dst = Some(dst);
    self
  }

  pub fn dst(&self) -> Option<&String> {
    self.dst.as_ref()
  }

  pub fn reset_dst(&mut self) {
    self.dst = None;
  }

  pub fn set_coins(&mut self, coins: String) {
    self.coins = Some(coins);
  }

  pub fn with_coins(mut self, coins: String) -> Apiv1exploreraddressOutputs {
    self.coins = Some(coins);
    self
  }

  pub fn coins(&self) -> Option<&String> {
    self.coins.as_ref()
  }

  pub fn reset_coins(&mut self) {
    self.coins = None;
  }

  pub fn set_uxid(&mut self, uxid: String) {
    self.uxid = Some(uxid);
  }

  pub fn with_uxid(mut self, uxid: String) -> Apiv1exploreraddressOutputs {
    self.uxid = Some(uxid);
    self
  }

  pub fn uxid(&self) -> Option<&String> {
    self.uxid.as_ref()
  }

  pub fn reset_uxid(&mut self) {
    self.uxid = None;
  }

}


