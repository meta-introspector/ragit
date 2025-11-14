use thiserror::Error;

#[derive(Debug, Error)]
pub enum PdlError {
    #[error("invalid pdl: {0}")]
    InvalidPdl(String),
    #[error("invalid turn separator: {0}")]
    InvalidTurnSeparator(String),
    #[error("role missing")]
    RoleMissing,
    #[error("invalid inline block")]
    InvalidInlineBlock,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    Tera(#[from] tera::Error),
    #[error("image type error: {0}")]
    ImageType(String),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Image(#[from] image::ImageError),
    #[error(transparent)]
    Base64(#[from] base64::DecodeError),
    #[error(transparent)]
    File(#[from] ragit_file_error::FileError),
    #[error("{0}")]
    Other(String),
}

impl From<String> for PdlError {
    fn from(s: String) -> Self {
        PdlError::Other(s)
    }
}

#[derive(Debug, Error)]
pub enum SchemaParseError {
    #[error("invalid schema: {0}")]
    InvalidSchema(String),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
