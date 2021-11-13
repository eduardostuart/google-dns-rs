# google-dns-rs

**google-dns-rs is a third party Google DNS client for rust.**

---


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
