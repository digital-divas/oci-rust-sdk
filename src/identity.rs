use chrono::{DateTime, Utc};
use reqwest::{header::HeaderMap, Response};

use crate::{base_client::oci_signer, config::AuthConfig};

pub struct Identity {
    config: AuthConfig,
    service_endpoint: String,
}

impl Identity {
    ///Creates a new `Identity` which is the client necessary to interact with this type of object on OCI.
    ///
    ///## Example 1
    ///```no_run
    ///use oci_sdk::{
    ///    config::AuthConfig,
    ///    identity::{Identity},
    ///};
    ///
    ///let auth_config = AuthConfig::from_file(None, None);
    ///let identity = Identity::new(auth_config, None);
    ///```
    ///
    /// ## Example 2
    ///
    ///```rust
    ///use oci_sdk::{
    ///    config::AuthConfig,
    ///    identity::{Identity},
    ///};
    ///
    ///let auth_config = AuthConfig::from_file(Some("tests/assets/oci_config".to_string()), Some("DEFAULT".to_string()));
    ///let identity = Identity::new(auth_config, None);
    ///```
    ///Returns the Nosql client.
    pub fn new(config: AuthConfig, service_endpoint: Option<String>) -> Identity {
        let se = service_endpoint.unwrap_or(format!(
            "https://identity.{}.oci.oraclecloud.com",
            config.region
        ));
        return Identity {
            config,
            service_endpoint: se,
        };
    }

    pub async fn get_current_user(
        &self,
    ) -> Result<Response, Box<dyn std::error::Error + Send + Sync>> {
        let client = reqwest::Client::new();

        let mut headers = HeaderMap::new();

        let now: DateTime<Utc> = Utc::now();
        headers.insert(
            "date",
            now.to_rfc2822().replace("+0000", "GMT").parse().unwrap(),
        );

        let path = format!("/20160918/users/{}", self.config.user);

        oci_signer(
            &self.config,
            &mut headers,
            String::from("get"),
            &path,
            &self.service_endpoint,
        );

        let response = client
            .get(format!("{}{}", self.service_endpoint, path))
            .headers(headers)
            .send()
            .await?;

        return Ok(response);
    }

    pub async fn get_user(
        &self,
        user_ocid: String,
    ) -> Result<Response, Box<dyn std::error::Error + Send + Sync>> {
        let client = reqwest::Client::new();

        let mut headers = HeaderMap::new();

        let now: DateTime<Utc> = Utc::now();
        headers.insert(
            "date",
            now.to_rfc2822().replace("+0000", "GMT").parse().unwrap(),
        );

        let path = format!("/20160918/users/{}", user_ocid);

        oci_signer(
            &self.config,
            &mut headers,
            String::from("get"),
            &path,
            &self.service_endpoint,
        );

        let response = client
            .get(format!("{}{}", self.service_endpoint, path))
            .headers(headers)
            .send()
            .await?;

        return Ok(response);
    }

    pub async fn list_users(
        &self,
        compartment_id: String,
    ) -> Result<Response, Box<dyn std::error::Error + Send + Sync>> {
        let client = reqwest::Client::new();

        let mut headers = HeaderMap::new();

        let now: DateTime<Utc> = Utc::now();
        headers.insert(
            "date",
            now.to_rfc2822().replace("+0000", "GMT").parse().unwrap(),
        );

        let path = format!("/20160918/users?compartmentId={}", compartment_id);

        oci_signer(
            &self.config,
            &mut headers,
            String::from("get"),
            &path,
            &self.service_endpoint,
        );

        let response = client
            .get(format!("{}{}", self.service_endpoint, path))
            // .query(&[("compartmentId", compartment_id)])
            .headers(headers)
            .send()
            .await?;

        return Ok(response);
    }
}
