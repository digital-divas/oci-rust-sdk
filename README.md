# oci-rust-sdk

[![Crates.io](https://img.shields.io/crates/v/oci-sdk.svg)](https://crates.io/crates/oci-sdk)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/digital-divas/oci-rust-sdk/blob/master/LICENSE)
[![Rust](https://github.com/digital-divas/oci-rust-sdk/actions/workflows/rust.yml/badge.svg)](https://github.com/digital-divas/oci-rust-sdk/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/digital-divas/oci-rust-sdk/branch/master/graph/badge.svg?token=XJJXHENTK4)](https://codecov.io/gh/digital-divas/oci-rust-sdk)

 Oracle Cloud Infrastructure SDK for Rust Lang 

## About

OCI-Rust-SDK is written to access the OCI API using async methods.

```rust
use oci_sdk::{
    config::AuthConfig,
    identity::Identity
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Set up auth config
    let auth_config = AuthConfig::from_file(
        Some("~/.oci/config".to_string()),
        Some("DEFAULT".to_string())
    );
    // Create a service client
    let identity = Identity::new(auth_config, None);
    //# Get the current user
    let response = identity.get_current_user().await?;
    // parse information
    let body = response.text().await?;

    println!("{}", body);
    // {
    //     "compartment_id": "ocid1.tenancy.oc1...",
    //     "description": "Test user",
    //     "id": "ocid1.user.oc1...",
    //     "inactive_status": null,
    //     "lifecycle_state": "ACTIVE",
    //     "name": "test-user@corp.com",
    //     "time_created": "2016-08-30T23:46:44.680000+00:00"
    // }

    Ok(())}
}
```

## Examples

You can look for the [test folder](./tests/) for more examples.

## Development

### OCI-Emulator

We recommend you to use [oci-emulator](https://github.com/cameritelabs/oci-emulator) to develop new features and testing.

To do so, just run:

```bash
docker run -d --name oci-emulator -p 12000:12000 cameritelabs/oci-emulator:latest
```

You can then use the `service_endpoint` parameter available on every client to use it. For example:

```rust
let auth_config = AuthConfig::from_file(None, None);

Nosql::new(auth_config, Some("http://localhost:12000".to_string()));
```

### Running Tests

We're using [tarpaulin](https://github.com/xd009642/tarpaulin) to generate code coverage.
To use it, you'll need to install it using cargo:

```bash
cargo install tarpaulin
```

After installing it, you can build/test and generate the coverage simply using:

```bash
cargo tarpaulin --out Lcov
```

We're using Lcov format to upload the coverage to `codecov`.
You can view the coverage on VSCode using [Coverage Gutters](https://marketplace.visualstudio.com/items?itemName=ryanluker.vscode-coverage-gutters).

If you don't want to generate coverage you can simply use:

```bash
cargo test
```
