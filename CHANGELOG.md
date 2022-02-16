# Changelog

This project follows semantic versioning.

Possible header types:

- `Features` for any new features added, or for backwards-compatible
  changes to existing functionality.
- `Bug Fixes` for any bug fixes.
- `Breaking Changes` for any backwards-incompatible changes.

## [Unreleased]

## v0.2.0 (2022-02-16)

### Features

* Add [`thiserror`] dependency to simplify the generation of library errors.
* Ensure library only raises `RustWistiaError` error types, and does *not* re-raise 
  errors from any internal dependencies.

[`thiserror`]: https://docs.rs/thiserror

## v0.1.0 (2022-02-15)

- Initial Release on [crates.io] :tada:

[crates.io]: https://crates.io/crates/rust-wistia
