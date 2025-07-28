use ragit_types::ApiError as Error;
use std::sync::Arc;

pub fn map_serde_json_error<T>(result: Result<T, serde_json::Error>) -> Result<T, Error> {
    result.map_err(|e| Error::JsonSerdeError(Arc::new(e)))
}
