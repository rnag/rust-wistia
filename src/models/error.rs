//! Library-specific errors, including types and implementations.
//!
use hyper::http::uri::InvalidUri;

use std::io;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Base error types raised by this library.
///
/// # Note
/// The `rust-wistia` crate should *only* ever raise the error kinds as
/// defined under here.
///
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum RustWistiaError {
    #[error("no such file: the source file path `{0}` should exist")]
    FileNotFound(String),
    #[error("environment variable `{name}` must be set")]
    EnvVarNotFound { name: String },
    /// An invalid media asset type is specified
    #[error("{video_id}: no such asset ({r#type}) for video.\n  Valid Assets: {valid_types:?}")]
    AssetNotFound {
        r#type: String,
        video_id: String,
        valid_types: Vec<String>,
    },
    /// A `media_id` or `media` argument is not specified
    #[error("An argument for `media_id` or `media` is required.")]
    MediaIsRequired,
    /// `RequestError` is raised when the Wistia API responds back with a
    /// *non-* "OK" response.
    ///
    /// More specifically, this error is raised when the status code of
    /// a response is between 400 and 600, which indicates its either a client
    /// error or a server error.
    ///
    /// # Note
    ///
    /// The `error` and `message` fields are mutually-exclusive; if we cannot
    /// de-serialize `error`, the `message` will be populated instead with the
    /// response data.
    ///
    #[error(
        "invalid request\n  status: {status_code:?})\n  reason: {reason}\n  error: {error:#?}"
    )]
    Request {
        status_code: u16,
        reason: String,
        error: WistiaError,
    },
    #[error("unknown rust-wistia error")]
    Unknown,
    // Transparent (pass-through) errors
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Hyper(#[from] hyper::Error),
    #[error(transparent)]
    Http(#[from] hyper::http::Error),
    #[error(transparent)]
    InvalidUri(#[from] InvalidUri),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SerdeUrlEncodedSer(#[from] serde_urlencoded::ser::Error),
}

/// An error returned from the Wistia API, along with a custom error
/// code from the Wistia side.
///
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct WistiaError {
    #[serde(default)]
    #[serde(rename = "error")]
    pub message: String,
    pub code: Option<String>,
    pub detail: Option<String>,
}
