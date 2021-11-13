# google-dns-rs

[![Crates.io](https://img.shields.io/crates/v/google-dns-rs.svg)](https://crates.io/crates/google-dns-rs)
[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

---

[Documentation](https://docs.rs/google-dns-rs/0.2.0/google_dns_rs/)

## Install

Add the following line to your Cargo.toml file:

```toml
google-dns-rs = "0.2.0"
```

## Usage

```rust
use google_dns_rs::api::{Dns, DoH, Result};

async fn github_dns_records() -> Result<Dns> {
    DoH::new("github.com".to_string())
        // .set_type(2) // NS record type
        // .set_cd(true) // disable or enable DNSSEC check
        // .set_ct("application/x-javascript".to_string()) // content type
        // .set_do(true) // include DNSSEC recods
        .resolve()
        .await
}
```

## License

[MIT](./LICENSE)
