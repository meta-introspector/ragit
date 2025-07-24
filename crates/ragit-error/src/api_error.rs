use thiserror::Error;
use ragit_fs::FileError;
use std::path::PathBuf;
use ragit_types::JsonType;
use ragit_types::api_error::TeraError as TypesTeraError;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("json type error: expected {expected}, got {got}")]
    JsonTypeError { expected: JsonType, got: JsonType },
    #[error("json object invalid field: {0}")]
    JsonObjectInvalidField(String),
    #[error("json object missing field: {0}")]
    JsonObjectMissingField(String),
    #[error("invalid model name: {name}, candidates: {candidates:?}")]
    InvalidModelName {
        name: String,
        candidates: Vec<String>,
    },
    #[error("invalid api provider: {0}")]
    InvalidApiProvider(String),
    #[error(transparent)]
    FileError(#[from] FileError),
    #[error("api key not found: {env_var:?}")]
    ApiKeyNotFound { env_var: Option<String> },
    #[error(transparent)]
    StdIoError(#[from] std::io::Error),
    #[error("cannot read image: {0}")]
    CannotReadImage(String /* model name */),

    /// If you see this error, there must be a bug in this library
    #[error("no try")]
    NoTry,

    /// see <https://docs.rs/reqwest/latest/reqwest/struct.Error.html>
    #[error("reqwest error: {0}")]
    ReqwestError(String),

    /// see <https://docs.rs/tera/latest/tera/struct.Error.html>
    #[error(transparent)]
    TeraError(#[from] TypesTeraError),

    #[error("wrong schema: {0}")]
    WrongSchema(String),
    #[error("server error: status_code: {status_code}, body: {body:?}")]
    ServerError {
        status_code: u16,
        body: String,
    },
    #[error("unsupported media format: {extension:?}")]
    UnsupportedMediaFormat { extension: Option<String> },
    #[error("test model")]
    TestModel,
    #[error("invalid test model: {0}")]
    InvalidTestModel(String),
    #[error("insufficient models")]
    InsufficientModels,
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),

    // New error variants
    #[error("model not selected")]
    ModelNotSelected,
    #[error("prompt missing: {0}")]
    PromptMissing(String),
    #[error("no such chunk: {0}")]
    NoSuchChunk(String),
    #[error("broken index: {0}")]
    BrokenIndex(String),
    #[error("no such file: {path:?}, uid: {uid:?}")]
    NoSuchFile { path: Option<String>, uid: Option<String> },
    #[error("index already exists at {0}")]
    IndexExists(PathBuf),
    #[error("not implemented: {0}")]
    NotImplemented(String),
    #[error(transparent)]
    TypesApiError(#[from] ragit_types::api_error::ApiError),
}

#[derive(Debug, Error)]
pub enum TeraError {
    #[error("dummy tera error")]
    Dummy,
}
