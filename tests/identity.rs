#[cfg(test)]
mod tests {
    use oci_sdk::{config::AuthConfig, identity::Identity};

    #[tokio::test]
    async fn get_current_user() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let auth_config = AuthConfig::from_file(Some("tests/assets/oci_config".to_string()), None);
        let identity = Identity::new(auth_config, Some("http://localhost:12000".to_string()));

        let response = identity.get_current_user().await?;
        let body = response.text().await?;

        println!("{}", body);

        Ok(())
    }

    #[tokio::test]
    async fn get_user() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let auth_config = AuthConfig::from_file(Some("tests/assets/oci_config".to_string()), None);
        let identity = Identity::new(auth_config, Some("http://localhost:12000".to_string()));

        let response = identity
            .get_user(
                "ocid1.user.oc1..aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                    .to_string(),
            )
            .await?;
        let body = response.text().await?;

        println!("{}", body);

        Ok(())
    }

    #[tokio::test]
    async fn list_users() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let auth_config = AuthConfig::from_file(Some("tests/assets/oci_config".to_string()), None);
        let identity = Identity::new(auth_config, Some("http://localhost:12000".to_string()));

        let response = identity
            .list_users(
                "ocid1.tenancy.oc1..aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                    .to_string(),
            )
            .await?;
        let body = response.text().await?;

        println!("{}", body);

        Ok(())
    }
}
