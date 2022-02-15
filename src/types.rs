/// Result type often returned from methods that can have hyper `Error`s.
pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Result<T> = std::result::Result<T, Error>;
