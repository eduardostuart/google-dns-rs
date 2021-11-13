# google-dns-rs

## Install

Add the following line to your Cargo.toml file:

```toml
google-dns-rs = "0.1.0"
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
