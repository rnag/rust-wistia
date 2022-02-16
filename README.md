# rust-wistia

[<img alt="github" src="https://img.shields.io/badge/github-rnag/rust-wistia?style=for-the-badge&labelColor=555555&logo=github" height="25">](https://github.com/rnag/rust-wistia)
[<img alt="crates.io" src="https://img.shields.io/crates/v/rust-wistia.svg?style=for-the-badge&color=fc8d62&logo=rust" height="25">](https://crates.io/crates/rust-wistia)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/rust-wistia/latest?style=for-the-badge&labelColor=555555" height="25">](https://docs.rs/rust-wistia)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/rnag/rust-wistia/build/main?style=for-the-badge" height="25">](https://github.com/rnag/rust-wistia/actions?query=branch%3Amain)

**rust-wistia** is a *rust crate* which provides an `async` wrapper API that lets you easily interact
with the [Wistia API](https://wistia.com/support/developers).

This is inspired by and based on [wystia], a Python wrapper library I created for the Wistia API.

[wystia]: https://github.com/rnag/wystia

---

This crate works with Cargo with a `Cargo.toml` like:

```toml
[dependencies]
rust-wistia = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Table of Contents

* [Getting Started](#getting-started)
* [Dependencies and Features](#dependencies-and-features)
* [Examples](#examples)
* [Contributing](#contributing)
* [License](#license)
* [Authors](#authors)

## Getting Started

Getting started with the `rust-wistia` library is easy:

1. Set **WISTIA_API_TOKEN** in your environment; you can
   also use the `from_token` constructor
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
rust-wistia = { version = "0.3", default-features = false, features = ["native-tls", "logging", "serde-std"] }
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
