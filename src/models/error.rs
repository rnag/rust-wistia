//! Library-specific errors, including types and implementations.
//!
use serde::{Deserialize, Serialize};

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
#[derive(Debug, Deserialize, Serialize)]
pub struct RequestError {
    pub status: u16,
    pub reason: String,
    pub error: WistiaError,
}

impl RequestError {
    /// Create a new `RequestError` object from a status code and reason.
    ///
    /// The `error` and `message` fields are mutually-exclusive, and so will
    /// both initially be unset.
    pub fn new(status: u16, reason: String) -> Self {
        Self {
            status,
            reason,
            // message: None,
            error: WistiaError::default(),
        }
    }
}
/// An error returned from the Wistia API, along with a custom error
/// code from the Wistia side.
///
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct WistiaError {
    #[serde(default)]
    pub error: String,
    pub code: Option<String>,
    pub detail: Option<String>,
}

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for RequestError {}
