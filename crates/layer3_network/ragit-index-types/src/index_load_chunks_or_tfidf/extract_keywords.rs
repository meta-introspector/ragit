use crate::prelude::*;
use crate::index_struct::Index;
use ragit_types::ApiError;
use ragit_utils::query::Keywords;

impl Index {
    pub async fn extract_keywords(&self, _query: &str) -> Result<Keywords, ApiError> {
        Err(ApiError::NotImplemented("extract_keywords".to_string()))
    }
}
