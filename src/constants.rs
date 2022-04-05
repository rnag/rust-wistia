//! Library-specific constants

/// API endpoint for the Wistia [Upload API]
///
/// [Upload API]: https://wistia.com/support/developers/upload-api
pub const UPLOAD_API: &str = "https://upload.wistia.com";

/// Environment variable to be used to retrieve the [API access token],
/// when `WistiaClient::from_env()` is invoked.
///
/// [API access token]: https://wistia.com/support/developers/data-api#getting-started
///
pub const ENV_VAR_NAME: &str = "WISTIA_API_TOKEN";

/// Default file name when uploading a file stream (via the `StreamUploader`)
/// via the Upload API.
pub const DEFAULT_FILENAME: &str = "file.mp4";
