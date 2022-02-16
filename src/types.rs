//! Library-specific type definitions
//!
use crate::models::RustWistiaError;

/// Result type with errors populated solely by *our* library.
pub type Result<T> = std::result::Result<T, RustWistiaError>;
