//! Common result and error types

use std::fmt::Display;

/// Application result type
pub type ChiselResult<T> = Result<T, ChiselError>;

/// Enumeration covering various different common errors that might arise
#[derive(Debug, Clone)]
pub enum ChiselError {
    /// An invalid file has been specified
    InvalidFile,
    /// Invalid input has been specified
    InvalidInput,
    /// Expecting piped input, but stdin is actually a TTY
    NoPipedInput,
    /// Not a TTY
    NoTty,
    /// Failed to send to the rendering pipeline
    RenderPipelineSendFailed,
}

impl Display for ChiselError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFile => write!(f, "An invalid file has been specified"),
            Self::InvalidInput => write!(f, "Invalid, or junk input has been specified"),
            Self::NoPipedInput => write!(
                f,
                "Expecting piped input, but doesn't look like there is any..."
            ),
            Self::NoTty => write!(f, "Not a tty!"),
            Self::RenderPipelineSendFailed => write!(f, "Failed to send command list to renderer"),
        }
    }
}
