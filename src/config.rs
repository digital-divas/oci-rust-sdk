use configparser::ini::Ini;
use openssl::pkey::{PKey, Private};
use openssl::rsa::Rsa;
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
        let key = fs::read_to_string(&key_file).expect("key_file doest not exists");

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

        let config_content =
            fs::read_to_string(&fp).expect(&format!("config file '{}' doest not exists", fp));

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
