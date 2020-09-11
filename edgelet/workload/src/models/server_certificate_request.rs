/* 
 * IoT Edge Module Workload API
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 2020-07-22
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerCertificateRequest {
  /// Subject common name
  #[serde(rename = "commonName")]
  common_name: String,
  /// Certificate expiration date-time (ISO 8601)
  #[serde(rename = "expiration")]
  expiration: String
}

impl ServerCertificateRequest {
  pub fn new(common_name: String, expiration: String) -> ServerCertificateRequest {
    ServerCertificateRequest {
      common_name: common_name,
      expiration: expiration
    }
  }

  pub fn set_common_name(&mut self, common_name: String) {
    self.common_name = common_name;
  }

  pub fn with_common_name(mut self, common_name: String) -> ServerCertificateRequest {
    self.common_name = common_name;
    self
  }

  pub fn common_name(&self) -> &String {
    &self.common_name
  }


  pub fn set_expiration(&mut self, expiration: String) {
    self.expiration = expiration;
  }

  pub fn with_expiration(mut self, expiration: String) -> ServerCertificateRequest {
    self.expiration = expiration;
    self
  }

  pub fn expiration(&self) -> &String {
    &self.expiration
  }


}



