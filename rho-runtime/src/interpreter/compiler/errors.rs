
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


}


#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("Process variable `{0}` is used as a name variable")]
    UnexpectedNameContext(String),


    #[error("Free variable `{0}` is used twice as a binder in name context")]
    UnexpectedReuseOfNameContextFree(String),

}