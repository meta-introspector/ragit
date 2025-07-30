use crate::index_struct::Index;
use ragit_types::ApiError;
use ragit_utils::query::Keywords;

impl Index {
    pub async fn extract_keywords(&self, query: String) -> Result<Keywords, ApiError> {
        eprintln!("Placeholder for extract_keywords: query={}", query);
        Ok(Keywords::new())
    }
}