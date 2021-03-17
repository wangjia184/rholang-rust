
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompliationError {
    #[error("Supplied source string contain an internal 0 byte")]
    NulSourceError(#[from] std::ffi::NulError),

    #[error("Unable to parse source")]
    SourceUtf8Error(#[from] std::str::Utf8Error),

    #[error("Unrecognized token `{0}`")]
    UnrecognizedToken(u32),

    #[error("Unrecognized kind `{0}` in {1}")]
    UnrecognizedKind(u32, String),

    #[error("Null pointer in {0}.")]
    NullPointer(String),

    #[error("`fmemopen()` failed")]
    FileMemOpenFailed,

    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    
    #[error("unknown error")]
    Unknown,
}