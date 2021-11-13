//! # google-dns-rs
//! google-dns-rs is a third party Google DNS client for rust.
//!
//! ```no_run
//! use google_dns_rs::api::{Dns, DoH, Result};
//! async fn github_dns_records() -> Result<Dns> {
//!     DoH::new("github.com".to_string())
//!         // .set_type(2) // NS record type
//!         // .set_cd(true) // disable or enable DNSSEC check
//!         // .set_ct("application/x-javascript".to_string()) // content type
//!         // .set_do(true) // include DNSSEC recods
//!         .resolve()
//!         .await
//! }
//! ```

pub mod api;
pub mod error;
