use ragit_fs::FileError;
use ragit_pdl::JsonType;

#[derive(Debug, thiserror::Error)]
pub enum Error {
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
    PdlError(#[from] ragit_pdl::Error),
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
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    /// see <https://docs.rs/serde_json/latest/serde_json/struct.Error.html>
    #[error(transparent)]
    JsonSerdeError(#[from] serde_json::Error),

    /// see <https://docs.rs/tera/latest/tera/struct.Error.html>
    #[error(transparent)]
    TeraError(#[from] tera::Error),

    #[error("wrong schema: {0}")]
    WrongSchema(String),
    #[error("server error: status_code: {status_code}, body: {body:?}")]
    ServerError {
        status_code: u16,
        body: Result<String, reqwest::Error>,
    },
    #[error("unsupported media format: {extension:?}")]
    UnsupportedMediaFormat { extension: Option<String> },
    #[error("test model")]
    TestModel,
    #[error("insufficient models")]
    InsufficientModels,
}
