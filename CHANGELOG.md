# Changelog

This project follows semantic versioning.

Possible header types:

- `Features` for any new features added, or for backwards-compatible
  changes to existing functionality.
- `Bug Fixes` for any bug fixes.
- `Breaking Changes` for any backwards-incompatible changes.

## [Unreleased]

## v0.4.0 (2022-04-04)

### Features

* Add new method `DataClient::update_media()` to update info on a Wistia video

## v0.3.0 (2022-04-04)

### Features

* Update examples to use `clap` for parsing arguments
* Add new struct `DataClient` (aliased to `WistiaClient`) for interacting with
  the Wistia Data API
* Add new method `DataClient::get_media()` to retrieve info on a Wistia video

## v0.2.2 (2022-02-24)

### Features

* Replace usage of `&'static str` with `&str`
* Add `from()` constructors for API clients so easier to pass in access tokens if needed.
* Add new `UrlUploader::url()` fluent method, which can be useful when the
  `from()` constructor is used.

## v0.2.1 (2022-02-16)

### Bug Fixes

* Ensure `examples/assets` directory isn't included when publishing the crate to *crates.io*.

## v0.2.0 (2022-02-16)

### Features

* Add [`thiserror`] dependency to simplify the generation of library errors.
* Ensure library only raises `RustWistiaError` error types, and does *not* re-raise 
  errors from any internal dependencies.

[`thiserror`]: https://docs.rs/thiserror

## v0.1.0 (2022-02-15)

- Initial Release on [crates.io] :tada:

[crates.io]: https://crates.io/crates/rust-wistia
