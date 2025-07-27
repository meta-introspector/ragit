
//use ragit_index_types::Index;
use ragit_types::ApiError;
use ragit_utils::query::Keywords;
//use ragit_index_types::Index;
use crate::prelude::Index;
pub async fn extract_keywords(
    _index: &Index,
    _query: &str,
) -> Result<Keywords, ApiError> {
    Err(ApiError::NotImplemented("extract_keywords".to_string()))
}
