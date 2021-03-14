
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompliationError {
    #[error("Supplied source string contain an internal 0 byte")]
    NulSourceError(#[from] std::ffi::NulError),


    #[error("the data for key `{0}` is not available")]
    Redaction(String),

    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    
    #[error("unknown error")]
    Unknown,
}