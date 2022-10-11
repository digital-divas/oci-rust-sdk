#[cfg(test)]
mod tests {
    use oci_sdk::{AuthConfig, CreateTableDetails, Nosql, QueryDetails, TableLimits};

    #[tokio::test]
    async fn nosql() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let auth_config = AuthConfig::from_file(None, None);
        let nosql = Nosql::new(auth_config, Some("http://localhost:12000".to_string()));

        let table_limits = TableLimits {
            max_read_units: 1,
            max_write_units: 1,
            max_storage_in_g_bs: 1,
        };

        let create_table_details = CreateTableDetails {
        name: String::from("table_name"),
        compartment_id: String::from("ocid1.compartment.oc1..randomcompartment"),
        ddl_statement: String::from("CREATE TABLE table_name ( stream_name string, start number, finish number, shot_source string DEFAULT \"IMAGE_SERVER\" NOT NULL, video_source string DEFAULT \"OCI\" NOT NULL, PRIMARY KEY ( SHARD ( stream_name ), start ) )"),
        table_limits,
        };

        let response = nosql.create_table(create_table_details).await?;
        let body = response.text().await?;

        println!("{}", body);

        let query_details = QueryDetails {
            compartment_id: String::from("ocid1.compartment.oc1..randomcompartment"),
            statement: String::from("SELECT * FROM table_name"),
        };

        let response = nosql.query(query_details, 1000).await?;

        let body = response.text().await?;

        println!("{}", body);

        Ok(())
    }
}
