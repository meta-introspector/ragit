use crate::uid::Uid;
pub use ragit_api::Error as ApiError;
pub use ragit_pdl::JsonType;
pub use ragit_fs::FileError;
    
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("internal error: {0}")]
    Internal(String),
    #[error("json type error: expected {expected}, got {got}")]
    JsonTypeError {
        expected: JsonType,
        got: JsonType,
    },
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    JsonSerdeError(#[from] serde_json::Error),
    #[error(transparent)]
    ImageError(#[from] image::ImageError),
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    #[error(transparent)]
    JoinError(#[from] tokio::task::JoinError),
    #[error(transparent)]
    FileError(#[from] FileError),
    #[error(transparent)]
    StdIoError(#[from] std::io::Error),
    #[error("from utf8 error")]
    FromUtf8Error,
    #[error(transparent)]
    ApiError(#[from] ApiError),
    #[error(transparent)]
    PdlError(#[from] ragit_pdl::Error),
    #[error("broken inverted index: {0}")]
    BrokenII(String),
    #[error("no such chunk: {0}")]
    NoSuchChunk(Uid),
    #[error("no such file: {path:?}, {uid:?}")]
    NoSuchFile { path: Option<PathBuf>, uid: Option<Uid> },
    #[error("broken index: {0}")]
    BrokenIndex(String),
    #[error("invalid config key: {0}")]
    InvalidConfigKey(String),
    #[error("prompt missing: {0}")]
    PromptMissing(String),
    #[error("index already exists: {0}")]
    IndexAlreadyExists(PathBuf),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error(transparent)]
    ParseBoolError(#[from] std::str::ParseBoolError),
    #[error(transparent)]
    ParseFloatError(#[from] std::num::ParseFloatError),
}


