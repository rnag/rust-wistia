# rust-wistia

[<img alt="github" src="https://img.shields.io/badge/github-rnag/rust--wistia-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="22">](https://github.com/rnag/rust-wistia)
[<img alt="crates.io" src="https://img.shields.io/crates/v/rust-wistia.svg?style=for-the-badge&color=fc8d62&logo=rust" height="22">](https://crates.io/crates/rust-wistia)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/rust-wistia/latest?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="22">](https://docs.rs/rust-wistia)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/rnag/rust-wistia/build/main?style=for-the-badge" height="22">](https://github.com/rnag/rust-wistia/actions?query=branch%3Amain)

**rust-wistia** is a *rust crate* which provides an `async` wrapper API that lets you easily interact
with the [Wistia API](https://wistia.com/support/developers).

This is inspired in part by [wystia], a Python library I created for the Wistia API.

[wystia]: https://github.com/rnag/wystia

---

This crate works with Cargo with a `Cargo.toml` like:

```toml
[dependencies]
rust-wistia = "0.5"
tokio = { version = "1", features = ["full"] }
```

## Table of Contents

* [Getting Started](#getting-started)
* [Examples](#examples)
* [Dependencies and Features](#dependencies-and-features)
* [Contributing](#contributing)
* [License](#license)
* [Authors](#authors)

## Getting Started

Getting started with the `rust-wistia` library is easy:

1. Set **WISTIA_API_TOKEN** in your environment; you can
   also use the `from` constructor
   to explicitly set the token value.
   Find out more  about [Authentication and Access Tokens](https://wistia.com/support/developers/data-api#creating-and-managing-access-tokens)
   in the Wistia API Documentation.

2. Add some usage to your application.

   Here's an example of uploading a media, via a public URL link:

   ```rust
   use rust_wistia::UrlUploader;

   #[tokio::main]
   async fn main() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
       let res = UrlUploader::new("my-url-link")?
           .name("My Video Name")
           .send()
           .await?;

       println!("Response: {res:#?}");

       // Print out some useful attributes
       println!("Video ID: {}", res.hashed_id);

       Ok(())
   }
   ```

## Examples

You can check out sample usage of API methods in the [examples/](https://github.com/rnag/rust-wistia/tree/main/examples)
folder in the project repo on GitHub.

## Dependencies and Features

This library uses only the minimum required dependencies, in order
to keep the overall size small. This crate uses [`hyper`][] and [`hyper-rustls`][]
internally, to make HTTPS requests to the Wistia API.

While `hyper-rustls` was chosen as the default TLS implementation
because it works without issue when cross-compiling for the
**x86_64-unknown-linux-musl** target as is common for [AWS Lambda][]
deployments, it is still possible to instead use the native [`hyper-tls`][]
implementation, which relies on OpenSSL.

To do this, disable the default "rust-tls" feature and enable the "native-tls" feature:

```toml
[dependencies]
rust-wistia = { version = "*", default-features = false, features = ["native-tls", "logging", "serde-std"] }
```

[`hyper`]: https://docs.rs/hyper
[`hyper-rustls`]: https://docs.rs/hyper-rustls
[`hyper-tls`]: https://docs.rs/hyper-tls
[AWS Lambda]: https://docs.aws.amazon.com/sdk-for-rust/latest/dg/lambda.html

## Contributing

Contributions are welcome! Open a pull request to fix a bug, or [open an issue][]
to discuss a new feature or change.

Check out the [Contributing][] section in the docs for more info.

[Contributing]: CONTRIBUTING.md
[open an issue]: https://github.com/rnag/rust-wistia/issues

## License

This project is proudly licensed under the MIT license ([LICENSE](LICENSE)
or http://opensource.org/licenses/MIT).

`rust-wistia` can be distributed according to the MIT license. Contributions
will be accepted under the same license.

## Authors

* [Ritvik Nag](https://github.com/rnag)
