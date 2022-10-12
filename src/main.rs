mod lib;

use crate::lib::{AuthConfig, CreateTableDetails, Nosql, QueryDetails, TableLimits};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let auth_config = AuthConfig::from_file(Some("tests/assets/oci_config".to_string()), None);
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

// {
//     "method": "POST",
//     "url": "http://localhost:12000/20190828/query?limit=1000",
//     "headers": {
//         "user-agent": "Oracle-PythonSDK/2.32.1 (python 3.9.7; x86_64-Linux)",
//         "accept-encoding": "gzip, deflate",
//         "accept": "application/json",
//         "connection": "keep-alive",
//         "content-type": "application/json",
//         "opc-client-info": "Oracle-PythonSDK/2.32.1",
//         "opc-request-id": "833D0CBCFA6547F6A94B5596EB7FBD6B",
//         "content-length": "103",
//         "date": "Sat, 23 Jul 2022 16:31:12 GMT",
//         "host": "localhost:12000",
//         "x-content-sha256": "zlP9czVeI11h9GVlklc/ptuhBjZLQtbCWpdKe663KSE=",
//         "authorization": 'Signature algorithm="rsa-sha256",headers="date (request-target) host content-length content-type x-content-sha256",keyId="ocid1.tenancy.oc1..random/ocid1.user.oc1..random/50:a6:c1:a1:da:71:57:dc:87:ae:90:af:9c:38:99:67",signature="ucLG7pN4egDwKgv9Rxc3kFLHq+rkF+BCNO/k2CgnASsPY+xObOU9BvCEkw7zOxD3B2UykeH2OHJ/76F5x7xZhEkFlRWju+fIgWo48m9d/lxTJ0NtfCoVZ8kZ7bNzg46NoMV++yMW6veKTL+skKYf00ybARs0KUTCsbEUL+yyXhcihrjX5vsG316vTfPDjZNOuWS/N5MCHIF8eSPlKrx5AiumEfFjhBaS8db49fxvb0l5TpMuArRZ0UydYH4LFh0YnHp455OnStL1NXSvSytpp5LOhePPxspCYTZoUTHVMO6A74Nvv5whKqpd366n2JouX0jgRAwVAALwRob+i7ns/g==",version="1"',
//     },
//     "_cookies": "",
//     "body": '{"compartmentId": "ocid1.compartment.oc1..randomcompartment", "statement": "SELECT * FROM table_name "}',
//     "hooks": {"response": []},
//     "_body_position": None,
// }
