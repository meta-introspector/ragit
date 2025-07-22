use crate::uid::Uid;
pub use ragit_api::Error as ApiError;
pub use ragit_pdl::JsonType;
use ragit_fs::FileError;
use std::string::FromUtf8Error;

pub type Path = String;

#[derive(Debug)]
pub enum Error {
    CliError {
        message: String,
        span: (String, usize, usize),
    },
    Internal(String),
    RagitUtilsError(ragit_utils::error::Error),
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Error::RagitUtilsError(ragit_utils::error::Error::AnyhowError(e))
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::RagitUtilsError(ragit_utils::error::Error::ReqwestError(e))
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::RagitUtilsError(ragit_utils::error::Error::JsonSerdeError(e))
    }
}

impl From<image::ImageError> for Error {
    fn from(e: image::ImageError) -> Error {
        Error::RagitUtilsError(ragit_utils::error::Error::ImageError(e))
    }
}

#[cfg(feature = "csv")]
impl From<csv::Error> for Error {
    fn from(e: csv::Error) -> Error {
        Error::RagitUtilsError(ragit_utils::error::Error::CsvError(e))
    }
}

impl From<FileError> for Error {
    fn from(e: FileError) -> Error {
        Error::RagitUtilsError(ragit_utils::error::Error::FileError(e))
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::RagitUtilsError(ragit_utils::error::Error::StdIoError(e))
    }
}

impl From<FromUtf8Error> for Error {
    fn from(_: FromUtf8Error) -> Error {
        Error::RagitUtilsError(ragit_utils::error::Error::FromUtf8Error)
    }
}

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Error {
        Error::RagitUtilsError(ragit_utils::error::Error::UrlParseError(e))
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(e: tokio::task::JoinError) -> Error {
        Error::RagitUtilsError(ragit_utils::error::Error::JoinError(e))
    }
}

#[cfg(feature = "pdf")]
impl From<mupdf::Error> for Error {
    fn from(e: mupdf::Error) -> Self {
        Error::RagitUtilsError(ragit_utils::error::Error::MuPdfError(e))
    }
}

#[cfg(feature = "svg")]
impl From<resvg::usvg::Error> for Error {
    fn from(e: resvg::usvg::Error) -> Self {
        Error::RagitUtilsError(ragit_utils::error::Error::UsvgError(e))
    }
}

#[cfg(feature = "svg")]
impl From<png::EncodingError> for Error {
    fn from(e: png::EncodingError) -> Self {
        Error::RagitUtilsError(ragit_utils::error::Error::PngEncodingError(e))
    }
}

impl From<tera::Error> for Error {
    fn from(e: tera::Error) -> Error {
        Error::RagitUtilsError(ragit_utils::error::Error::TeraError(e))
    }
}

impl From<ApiError> for Error {
    fn from(e: ApiError) -> Self {
        Error::RagitUtilsError(ragit_utils::error::Error::ApiError(e))
    }
}

impl From<ragit_pdl::Error> for Error {
    fn from(e: ragit_pdl::Error) -> Self {
        Error::RagitUtilsError(ragit_utils::error::Error::PdlError(e))
    }
}

impl From<ragit_cli::Error> for Error {
    fn from(e: ragit_cli::Error) -> Self {
        Error::CliError {
            message: e.kind.render(),
            span: e.span.unwrap_rendered(),
        }
    }
}

impl From<ragit_pdl::SchemaParseError> for Error {
    fn from(e: ragit_pdl::SchemaParseError) -> Self {
        Error::RagitUtilsError(ragit_utils::error::Error::PdlSchemaParseError(e))
    }
}

impl From<ragit_utils::error::Error> for Error {
    fn from(e: ragit_utils::error::Error) -> Self {
        Error::RagitUtilsError(e)
    }
}
