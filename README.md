# google-dns-rs

## Install

Add the following line to your Cargo.toml file:

```toml
google-dns-rs = "0.1.0"
```

## Usage

```rust
use google_dns_rs::api::{Dns, DoH, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let github = DoH::new("github.com".to_string()).resolve().await?;
    println!("{:#?}", github);
    Ok(())
}
```

## License

MIT
