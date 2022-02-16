// #![deny(warnings)]
// #![warn(rust_2018_idioms)]

//! An async Rust library implementation to interact with the
//! [Wistia API](https://wistia.com/support/developers).
//!
//! ## Example
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
//! ## Dependencies and Features
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
