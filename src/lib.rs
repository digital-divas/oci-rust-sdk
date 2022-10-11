use chrono::{DateTime, Utc};
use configparser::ini::Ini;
use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private};
use openssl::rsa::Rsa;
use openssl::sign::Signer;
use reqwest::{header::HeaderMap, Response};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::fs;

pub struct AuthConfig {
    pub user: String,
    pub fingerprint: String,
    pub tenancy: String,
    pub region: String,
    pub keypair: PKey<Private>,
}

impl AuthConfig {
    pub fn new(
        user: String,
        key_file: String,
        fingerprint: String,
        tenancy: String,
        region: String,
        passphrase: String,
    ) -> AuthConfig {
        let key = fs::read_to_string(&key_file).expect("file doest not exists");

        let keypair =
            Rsa::private_key_from_pem_passphrase(key.as_bytes(), passphrase.as_bytes()).unwrap();
        let keypair = PKey::from_rsa(keypair).unwrap();

        return AuthConfig {
            user,
            fingerprint,
            tenancy,
            region,
            keypair,
        };
    }

    pub fn from_file(file_path: Option<String>, profile_name: Option<String>) -> AuthConfig {
        let fp;
        let pn = profile_name.unwrap_or("DEFAULT".to_string());

        if file_path.is_none() {
            let home_dir_path = home::home_dir().expect("Impossible to get your home dir!");

            fp = format!(
                "{}/.oci/config",
                home_dir_path.to_str().expect("null value")
            );
        } else {
            fp = file_path.expect("file path is not string");
        }

        let config_content = fs::read_to_string(&fp).expect("config file doest not exists");

        let mut config = Ini::new();
        config
            .read(String::from(config_content))
            .expect("invalid config file");

        return AuthConfig::new(
            config.get(&pn, "user").unwrap(),
            config.get(&pn, "key_file").unwrap(),
            config.get(&pn, "fingerprint").unwrap(),
            config.get(&pn, "tenancy").unwrap(),
            config.get(&pn, "region").unwrap(),
            config.get(&pn, "passphrase").unwrap_or("".to_string()),
        );
    }
}

pub struct QueryDetails {
    pub compartment_id: String,
    pub statement: String,
}

pub struct Nosql {
    config: AuthConfig,
    service_endpoint: String,
}

pub struct TableLimits {
    pub max_read_units: u16,
    pub max_write_units: u16,
    pub max_storage_in_g_bs: u16,
}

pub struct CreateTableDetails {
    pub name: String,
    pub compartment_id: String,
    pub ddl_statement: String,
    pub table_limits: TableLimits,
}

fn encode_body(body: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(body);
    let result = hasher.finalize();
    let b64 = base64::encode(result);
    return b64;
}

fn oci_signer(
    config: &AuthConfig,
    headers: &mut HeaderMap,
    method: String,
    path: &String,
    host: &String,
) {
    let date = headers.get("date").unwrap();
    let date = date.to_str().unwrap();

    let host = host.replace("http://", "").replace("https://", "");

    let mut data = format!(
        "date: {}\n(request-target): {} {}\nhost: {}",
        date, method, path, host
    );

    let mut headers_auth = String::from("date (request-target) host");

    if headers.contains_key("content-length") {
        let content_length = headers.get("content-length").unwrap();
        let content_length = content_length.to_str().unwrap();
        data = format!("{}\ncontent-length: {}", data, content_length);
        headers_auth = format!("{} content-length", headers_auth)
    }

    if headers.contains_key("content-type") {
        let content_type = headers.get("content-type").unwrap();
        let content_type = content_type.to_str().unwrap();
        data = format!("{}\ncontent-type: {}", data, content_type);
        headers_auth = format!("{} content-type", headers_auth)
    }

    if headers.contains_key("x-content-sha256") {
        let content_sha256 = headers.get("x-content-sha256").unwrap();
        let content_sha256 = content_sha256.to_str().unwrap();
        data = format!("{}\nx-content-sha256: {}", data, content_sha256);
        headers_auth = format!("{} x-content-sha256", headers_auth)
    }

    let mut signer = Signer::new(MessageDigest::sha256(), &config.keypair).unwrap();
    signer.update(data.as_bytes()).unwrap();
    let signature = signer.sign_to_vec().unwrap();
    let b64 = base64::encode(signature);
    let key_id = format!("{}/{}/{}", config.tenancy, config.user, config.fingerprint);
    let authorization = format!("Signature algorithm=\"rsa-sha256\",headers=\"{}\",keyId=\"{}\",signature=\"{}\",version=\"1\"",headers_auth,key_id,b64);

    headers.insert("authorization", authorization.parse().unwrap());
}

impl Nosql {
    ///Creates a new `Nosql` which is the client necessary to interact with this type of object on OCI.
    ///
    ///## Example
    ///```rust
    ///use oci_sdk::{AuthConfig, Nosql};
    ///
    ///let auth_config = AuthConfig::from_file(None, None);
    ///let nosql = Nosql::new(auth_config, None);
    ///```
    ///Returns the Nosql client.
    pub fn new(config: AuthConfig, service_endpoint: Option<String>) -> Nosql {
        let se = service_endpoint.unwrap_or(format!(
            "https://nosql.{}.oci.oraclecloud.com",
            config.region
        ));
        return Nosql {
            config,
            service_endpoint: se,
        };
    }

    pub async fn create_table(
        &self,
        create_table_detais: CreateTableDetails,
    ) -> Result<Response, Box<dyn std::error::Error + Send + Sync>> {
        let client = reqwest::Client::new();

        let mut headers = HeaderMap::new();

        let body_json = json!({
          "name": &create_table_detais.name,
          "compartmentId": &create_table_detais.compartment_id,
          "ddlStatement": &create_table_detais.ddl_statement,
          "tableLimits": {
            "maxReadUnits": create_table_detais.table_limits.max_read_units,
            "maxWriteUnits": create_table_detais.table_limits.max_write_units,
            "maxStorageInGBs": create_table_detais.table_limits.max_storage_in_g_bs
          }
        });

        let body = body_json.to_string();

        let now: DateTime<Utc> = Utc::now();
        headers.insert(
            "date",
            now.to_rfc2822().replace("+0000", "GMT").parse().unwrap(),
        );
        headers.insert("x-content-sha256", encode_body(&body).parse().unwrap());
        headers.insert("content-length", body.len().to_string().parse().unwrap());
        headers.insert(
            "content-type",
            String::from("application/json").parse().unwrap(),
        );

        let path = format!("/20190828/tables");

        oci_signer(
            &self.config,
            &mut headers,
            String::from("post"),
            &path,
            &self.service_endpoint,
        );

        let response = client
            .post(format!("{}{}", self.service_endpoint, path))
            .body(body)
            .headers(headers)
            .send()
            .await?;

        return Ok(response);
    }

    pub async fn query(
        &self,
        query_details: QueryDetails,
        limit: u16,
    ) -> Result<Response, Box<dyn std::error::Error + Send + Sync>> {
        let client = reqwest::Client::new();

        let mut headers = HeaderMap::new();

        let body_json = json!({
          "compartmentId": &query_details.compartment_id,
          "statement": &query_details.statement,
        });

        let body = body_json.to_string();

        let now: DateTime<Utc> = Utc::now();
        headers.insert(
            "date",
            now.to_rfc2822().replace("+0000", "GMT").parse().unwrap(),
        );
        headers.insert("x-content-sha256", encode_body(&body).parse().unwrap());
        headers.insert("content-length", body.len().to_string().parse().unwrap());
        headers.insert(
            "content-type",
            String::from("application/json").parse().unwrap(),
        );

        let path = format!("/20190828/query?limit={}", limit);

        oci_signer(
            &self.config,
            &mut headers,
            String::from("post"),
            &path,
            &self.service_endpoint,
        );

        let response = client
            .post(format!("{}{}", self.service_endpoint, path))
            .body(body)
            .headers(headers)
            .send()
            .await?;

        return Ok(response);
    }
}
