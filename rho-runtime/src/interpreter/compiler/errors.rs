
use thiserror::Error;
use super::context::VarSort;

#[derive(Error, Debug)]
pub enum CompliationError {

    #[error("Supplied source string contain an internal 0 byte")]
    NulSourceError(#[from] std::ffi::NulError),

    #[error("Utf8Error occurs when parsing source at {0}:{1}. {2}")]
    SourceUtf8Error(i32, i32, std::str::Utf8Error),

    #[error("Unrecognized token `{0}`")]
    UnrecognizedToken(u32),

    #[error("Unrecognized kind `{0}` in {1}")]
    UnrecognizedKind(u32, String),

    #[error("Null pointer in {0}.")]
    NullPointer(String),

    #[error("Unsupported value {0}")]
    UnsupportedVarSort(VarSort),

    #[error("`fmemopen()` failed")]
    FileMemOpenFailed,

    

    #[error("Not implemented yet!")]
    NotImplemented,
}


#[derive(Error, Debug)]
pub enum SyntaxError {
    

    #[error("Invalid integer number `{0}`")]
    IntegerNumberError(String),


    #[error("Process variable `{0}` is used as a name variable")]
    UnexpectedNameContext(String),


    #[error("Free variable `{0}` is used twice as a binder in name context")]
    UnexpectedReuseOfNameContextFree(String),


}