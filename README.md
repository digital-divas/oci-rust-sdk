# oci-rust-sdk

[![Rust](https://github.com/digital-divas/oci-rust-sdk/actions/workflows/rust.yml/badge.svg)](https://github.com/digital-divas/oci-rust-sdk/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/digital-divas/oci-rust-sdk/branch/master/graph/badge.svg?token=XJJXHENTK4)](https://codecov.io/gh/digital-divas/oci-rust-sdk)

 Oracle Cloud Infrastructure SDK for Rust Lang 

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
