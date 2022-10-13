use openssl::hash::MessageDigest;
use openssl::sign::Signer;
use reqwest::header::HeaderMap;
use sha2::{Digest, Sha256};

use crate::config::AuthConfig;

pub fn encode_body(body: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(body);
    let result = hasher.finalize();
    let b64 = base64::encode(result);
    return b64;
}

pub fn oci_signer(
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
        headers_auth = format!("{} content-type", headers_auth);
    }

    if headers.contains_key("x-content-sha256") {
        let content_sha256 = headers.get("x-content-sha256").unwrap();
        let content_sha256 = content_sha256.to_str().unwrap();
        data = format!("{}\nx-content-sha256: {}", data, content_sha256);
        headers_auth = format!("{} x-content-sha256", headers_auth);
    }

    let mut signer = Signer::new(MessageDigest::sha256(), &config.keypair).unwrap();
    signer.update(data.as_bytes()).unwrap();
    let signature = signer.sign_to_vec().unwrap();
    let b64 = base64::encode(signature);
    let key_id = format!("{}/{}/{}", config.tenancy, config.user, config.fingerprint);
    let authorization = format!("Signature algorithm=\"rsa-sha256\",headers=\"{}\",keyId=\"{}\",signature=\"{}\",version=\"1\"",headers_auth,key_id,b64);

    headers.insert("authorization", authorization.parse().unwrap());
}
