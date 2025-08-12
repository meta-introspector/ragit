use thiserror::Error;
use super::token::Token;

#[derive(Debug, Error)]
pub enum SchemaParseError {
    #[error("unexpected byte: {0}")]
    UnexpectedByte(u8),
    #[error("unmatched group: {0}")]
    UnmatchedGroup(u8), // an opening delim
    #[error("unexpected token: {0:?}")]
    UnexpectedToken(Token),
    #[error("unexpected eof")]
    UnexpectedEof,
    #[error(transparent)]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error(transparent)]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("invalid constraint: {0}")]
    InvalidConstraint(String),
}
