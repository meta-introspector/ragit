use thiserror::Error;
use ragit_fs::FileError;

//use ragit_types::{ApiError as TypesApiError, JsonType};
use ragit_types::{ApiError as TypesApiError, JsonType};
use ragit_types::api_error::TeraError as TypesTeraError;
use ragit_utils::prelude::Error as RagitUtilsError;
use ragit_pdl::{SchemaParseError, Error as PdlError};

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    TypesApiError(#[from] TypesApiError),

    // From implementations for other error types
    #[error(transparent)]
    RagitUtilsError(#[from] RagitUtilsError),
    #[error(transparent)]
    SchemaParseError(#[from] SchemaParseError),
    #[error(transparent)]
    PdlError(#[from] PdlError),
    #[error(transparent)]
    JsonSerdeError(#[from] serde_json::Error),
    //#[error(transparent)]
    //JsonTypeError(#[from] TypesApiError),

    
    // copied in from types/api error for simplicity, this is createing duplicate code.
    #[error("insufficient models")]
    InsufficientModels,

    #[error("json type error: expected {expected}, got {got}")]
    JsonTypeError { expected: JsonType, got: JsonType },

    #[error(transparent)]
    FileError(#[from] FileError),
}

#[derive(Debug, Error)]
pub enum TeraError {
    #[error(transparent)]
    TypesTeraError(#[from] TypesTeraError),
}
