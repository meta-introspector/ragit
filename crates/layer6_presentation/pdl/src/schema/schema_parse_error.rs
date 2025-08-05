use thiserror::Error;

#[derive(Debug, Error)]
pub enum SchemaParseError {
    #[error("Unexpected byte: {0}")]
    UnexpectedByte(u8),
    #[error("Unmatched group: {0}")]
    UnmatchedGroup(u8),
    #[error("Parse int error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Parse float error: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}