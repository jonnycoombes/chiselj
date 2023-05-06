//! Common result and error types

/// Application result type
pub type ChiselResult<T> = Result<T, ChiselError>;

/// Enumeration covering various different common errors that might arise
pub enum ChiselError {
    /// A catch-all general error type
    GeneralError(String),
    /// An invalid file has been specified
    InvalidFile,
    /// Invalid input has been specified
    InvalidInput,
}
