// #![deny(warnings)]
// #![warn(rust_2018_idioms)]

//! [![github]](https://github.com/rnag/rust-wistia)&ensp;[![crates-io]](https://crates.io/crates/rust-wistia)&ensp;[![docs-rs]](https://docs.rs/rust-wistia)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! An async Rust library implementation to interact with the
//! [Wistia API](https://wistia.com/support/developers).
//!
//! <br>
//!
//! # Example
//!
//! ```no_run
//! use rust_wistia::UrlUploader;
//!
//! #[tokio::main]
//! async fn main() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     let res = UrlUploader::new("my-url-link")?
//!         .name("My Video Name")
//!         .send()
//!         .await?;
//!
//!     println!("Response: {res:#?}");
//!
//!     // Print out some useful attributes
//!     println!("Video ID: {}", res.hashed_id);
//!
//!     Ok(())
//! }
//! ```
//!
//! # Dependencies and Features
//!
//! This library uses only the minimum required dependencies, in order
//! to keep the overall size small. This crate uses [hyper] and
//! [hyper-rustls] internally, to make HTTPS requests to the Wistia API.
//!
//! While `hyper-rustls` was chosen as the default TLS implementation
//! because it works without issue when cross-compiling for the
//! **x86_64-unknown-linux-musl** target as is common for [AWS Lambda][]
//! deployments, it is still possible to instead use the native [`hyper-tls`][]
//! implementation, which relies on OpenSSL.
//!
//! To do this, disable the default "rust-tls" feature and enable the "native-tls" feature:
//!
//! ```toml
//! [dependencies]
//! rust-wistia = { version = "0.1", default-features = false, features = ["native-tls", "logging", "serde-std"] }
//! ```
//!
//! [hyper]: https://docs.rs/hyper
//! [hyper-rustls]: https://docs.rs/hyper-rustls
//! [`hyper-tls`]: https://docs.rs/hyper-tls
//! [AWS Lambda]: https://docs.aws.amazon.com/sdk-for-rust/latest/dg/lambda.html
//!

mod api;
pub mod constants;
mod https;
pub mod models;
pub mod status;
pub mod types;
pub mod utils;

pub use api::*;
pub use types::*;

#[cfg(feature = "logging")]
mod log {
    pub use log::{debug, error, trace, warn};
}

#[cfg(not(feature = "logging"))]
mod log {
    macro_rules! debug      ( ($($tt:tt)*) => {{}} );
    macro_rules! error      ( ($($tt:tt)*) => {{}} );
    macro_rules! trace      ( ($($tt:tt)*) => {{}} );
    macro_rules! warning    ( ($($tt:tt)*) => {{}} );
    pub(crate) use {debug, error, trace, warning as warn};
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
