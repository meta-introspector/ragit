use crate::uid::Uid;
pub use ragit_api::Error as ApiError;
pub use ragit_pdl::JsonType;
use ragit_fs::FileError;
use std::string::FromUtf8Error;
use std::path::PathBuf;

pub type Path = String;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("internal error: {0}")]
    Internal(String),
    #[error("json type error: {expected}, got {got}")]
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
    #[error(transparent)]
    PdlSchemaParseError(#[from] ragit_pdl::SchemaParseError),
    #[error(transparent)]
    TeraError(#[from] tera::Error),
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
    #[error("model not selected")]
    ModelNotSelected,
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
    #[error(transparent)]
    UidError(#[from] ragit_uid::UidError),
    #[error(transparent)]
    CliError(#[from] ragit_utils::error::CliError),
    #[error("cannot push: {0}")]
    CannotPush(String),
    #[error("index not found")]
    IndexNotFound,
    #[error("feature not enabled: {action} requires feature {feature}")]
    FeatureNotEnabled { action: String, feature: String },
    #[error("invalid model name: {name}, candidates: {candidates:?}")]
    InvalidModelName { name: String, candidates: Vec<String> },
    #[error("deprecated config: {key}, {message}")]
    DeprecatedConfig { key: String, message: String },
    #[error("dirty knowledge base")]
    DirtyKnowledgeBase,
    #[cfg(feature = "csv")]
    #[error(transparent)]
    CsvError(#[from] csv::Error),
    #[cfg(feature = "pdf")]
    #[error(transparent)]
    MuPdfError(#[from] mupdf::Error),
    #[cfg(feature = "svg")]
    #[error(transparent)]
    UsvgError(#[from] resvg::usvg::Error),
    #[cfg(feature = "svg")]
    #[error(transparent)]
    PngEncodingError(#[from] png::EncodingError),
}

impl From<ragit_cli::Error> for Error {
    fn from(e: ragit_cli::Error) -> Self {
        Error::CliError(e)
    }
}

impl From<ragit_utils::error::Error> for Error {
    fn from(e: ragit_utils::error::Error) -> Self {
        match e {
            ragit_utils::error::Error::Internal(s) => Error::Internal(s),
            ragit_utils::error::Error::JsonTypeError { expected, got } => Error::JsonTypeError { expected, got },
            ragit_utils::error::Error::ReqwestError(e) => Error::ReqwestError(e),
            ragit_utils::error::Error::JsonSerdeError(e) => Error::JsonSerdeError(e),
            ragit_utils::error::Error::ImageError(e) => Error::ImageError(e),
            ragit_utils::error::Error::UrlParseError(e) => Error::UrlParseError(e),
            ragit_utils::error::Error::JoinError(e) => Error::JoinError(e),
            ragit_utils::error::Error::FileError(e) => Error::FileError(e),
            ragit_utils::error::Error::StdIoError(e) => Error::StdIoError(e),
            ragit_utils::error::Error::FromUtf8Error => Error::FromUtf8Error,
            ragit_utils::error::Error::ApiError(e) => Error::ApiError(e),
            ragit_utils::error::Error::PdlError(e) => Error::PdlError(e),
            ragit_utils::error::Error::PdlSchemaParseError(e) => Error::PdlSchemaParseError(e),
            ragit_utils::error::Error::TeraError(e) => Error::TeraError(e),
            ragit_utils::error::Error::BrokenII(s) => Error::BrokenII(s),
            ragit_utils::error::Error::NoSuchChunk(uid) => Error::NoSuchChunk(uid),
            ragit_utils::error::Error::NoSuchFile { path, uid } => Error::NoSuchFile { path, uid },
            ragit_utils::error::Error::BrokenIndex(s) => Error::BrokenIndex(s),
            ragit_utils::error::Error::InvalidConfigKey(s) => Error::InvalidConfigKey(s),
            ragit_utils::error::Error::PromptMissing(s) => Error::PromptMissing(s),
            ragit_utils::error::Error::ModelNotSelected => Error::ModelNotSelected,
            ragit_utils::error::Error::IndexAlreadyExists(path) => Error::IndexAlreadyExists(path),
            ragit_utils::error::Error::AnyhowError(e) => Error::AnyhowError(e),
            ragit_utils::error::Error::ParseIntError(e) => Error::ParseIntError(e),
            ragit_utils::error::Error::ParseBoolError(e) => Error::ParseBoolError(e),
            ragit_utils::error::Error::ParseFloatError(e) => Error::ParseFloatError(e),
            ragit_utils::error::Error::UidError(e) => Error::UidError(e),
            ragit_utils::error::Error::CliError(e) => Error::CliError(e),
            #[cfg(feature = "csv")]
            ragit_utils::error::Error::CsvError(e) => Error::CsvError(e),
            #[cfg(feature = "pdf")]
            ragit_utils::error::Error::MuPdfError(e) => Error::MuPdfError(e),
            #[cfg(feature = "svg")]
            ragit_utils::error::Error::UsvgError(e) => Error::UsvgError(e),
            #[cfg(feature = "svg")]
            ragit_utils::error::Error::PngEncodingError(e) => Error::PngEncodingError(e),
        }
    }
}


