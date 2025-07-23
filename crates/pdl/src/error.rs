use crate::schema::SchemaParseError;
use ragit_fs::FileError;
use std::string::FromUtf8Error;

mod json_type;

pub use json_type::JsonType;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("role missing")]
    RoleMissing,
    #[error("invalid pdl: {0}")]
    InvalidPdl(String),
    #[error("invalid turn separator: {0}")]
    InvalidTurnSeparator(String),
    #[error("invalid inline block")]
    InvalidInlineBlock,
    #[error("invalid image type: {0}")]
    InvalidImageType(String),
    #[error("invalid role: {0}")]
    InvalidRole(String),
    #[error(transparent)]
    FileError(#[from] FileError),
    #[error("utf8 error")]
    Utf8Error(FromUtf8Error),
    #[error(transparent)]
    SchemaParseError(#[from] SchemaParseError),
    #[error("json type error: expected {expected}, got {got}")]
    JsonTypeError { expected: JsonType, got: JsonType },

    /// see <https://docs.rs/serde_json/latest/serde_json/struct.Error.html>
    #[error(transparent)]
    JsonSerdeError(#[from] serde_json::Error),

    /// see <https://docs.rs/base64/latest/base64/enum.DecodeError.html>
    #[error(transparent)]
    Base64DecodeError(#[from] base64::DecodeError),

    /// see <https://docs.rs/tera/latest/tera/struct.Error.html>
    #[error(transparent)]
    TeraError(#[from] tera::Error),
    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
}
