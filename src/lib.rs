///! # google-dns-rs
///! google-dns-rs is a third party Google DNS client for rust.
///!
///! ```no_run
///! use google_dns_rs::api::{DoH, Dns, Result};
///! pub async fn resolve_github() -> Result<Dns> {
///!    // DNS Over HTTPS
///!    Ok(DoH::new("github.com".to_string()).resolve().await?)
///! }
///! ```
pub mod api;
pub mod error;
