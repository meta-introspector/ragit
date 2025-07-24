use ragit_index::Index;
use ragit_error::ApiError;
use ragit_types::query::Keywords;

pub async fn extract_keywords(
    _index: &Index,
    _query: &str,
) -> Result<Keywords, ApiError> {
    Err(ApiError::NotImplemented("extract_keywords".to_string()))
}
