use std::sync::Arc;
use std::path::Path;
use crate::ApiError;

pub fn map_io_error<T>(result: Result<T, std::io::Error>) -> Result<T, ApiError> {
    result.map_err(|e| ApiError::StdIoError(Arc::new(e)))
}

pub fn map_serde_json_error<T>(result: Result<T, serde_json::Error>) -> Result<T, ApiError> {
    result.map_err(|e| ApiError::JsonSerdeError(Arc::new(e)))
}

pub fn map_anyhow_error<T>(result: Result<T, anyhow::Error>) -> Result<T, ApiError> {
    result.map_err(|e| ApiError::AnyhowError(Arc::new(e)))
}

pub fn read_dir_to_api_error(path: &Path) -> Result<std::fs::ReadDir, ApiError> {
    std::fs::read_dir(path).map_err(|e| ApiError::StdIoError(Arc::new(e)))
}

pub fn read_to_string_to_api_error(path: &Path) -> Result<String, ApiError> {
    std::fs::read_to_string(path).map_err(|e| ApiError::StdIoError(Arc::new(e)))
}