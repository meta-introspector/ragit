use crate::index_struct::Index;
use ragit_types::ApiError;
use std::path::PathBuf;
use ragit_types::RemoveResult;

pub async fn index_remove(
    index: &mut Index,
) -> Result<RemoveResult, ApiError> {
    Ok(RemoveResult { removed_files: 0, removed_chunks: 0 })
}
