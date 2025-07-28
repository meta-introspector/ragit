use ragit_types::ApiError as Error;
use std::sync::Arc;
use serde::de::DeserializeOwned;

pub fn map_serde_json_error<T>(result: Result<T, serde_json::Error>) -> Result<T, Error> {
    result.map_err(|e| Error::JsonSerdeError(Arc::new(e)))
}

pub fn map_serde_json_from_str_error<T: DeserializeOwned>(s: &str) -> Result<T, Error> {
    serde_json::from_str(s).map_err(|e| Error::JsonSerdeError(Arc::new(e)))
}