use crate::index_struct::Index;
use ragit_error::ApiError;
use ragit_types::remove_result::RemoveResult;

impl Index {
    pub async fn remove(&mut self) -> Result<RemoveResult, ApiError> {
        eprintln!("Placeholder for remove");
        Ok(RemoveResult::default())
    }
}