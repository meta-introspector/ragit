use crate::index_struct::Index;
use ragit_error::ApiError;
use ragit_utils::query::Keywords;

pub async fn extract_keywords(
    _index: &Index,
    _query: &str,
) -> Result<Keywords, ApiError> {
    Err(ApiError::NotImplemented("extract_keywords".to_string()))
}
