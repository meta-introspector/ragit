use crate::uid::Uid;
pub use ragit_api::Error as ApiError;
pub use ragit_pdl::JsonType;
pub use ragit_fs::FileError;
use std::string::FromUtf8Error;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    Internal(String),
    ReqwestError(reqwest::Error),
    JsonSerdeError(serde_json::Error),
    ImageError(image::ImageError),
    UrlParseError(url::ParseError),
    JoinError(tokio::task::JoinError),
    FileError(FileError),
    StdIoError(std::io::Error),
    FromUtf8Error,
    ApiError(ApiError),
    PdlError(ragit_pdl::Error),
    BrokenII(String),
    NoSuchChunk(Uid),
    NoSuchFile { path: Option<PathBuf>, uid: Option<Uid> },
    BrokenIndex(String),
    InvalidConfigKey(String),
    PromptMissing(String),
    IndexAlreadyExists(PathBuf),
    AnyhowError(anyhow::Error), // Add this variant
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::ReqwestError(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::JsonSerdeError(e)
    }
}

impl From<image::ImageError> for Error {
    fn from(e: image::ImageError) -> Error {
        Error::ImageError(e)
    }
}

impl From<FileError> for Error {
    fn from(e: FileError) -> Error {
        Error::FileError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::StdIoError(e)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(_: FromUtf8Error) -> Error {
        Error::FromUtf8Error
    }
}

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Error {
        Error::UrlParseError(e)
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(e: tokio::task::JoinError) -> Error {
        Error::JoinError(e)
    }
}

impl From<ApiError> for Error {
    fn from(e: ApiError) -> Self {
        Error::ApiError(e)
    }
}

impl From<ragit_pdl::Error> for Error {
    fn from(e: ragit_pdl::Error) -> Self {
        Error::PdlError(e)
    }
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Error::AnyhowError(e)
    }
}
