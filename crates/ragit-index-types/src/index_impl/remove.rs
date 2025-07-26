use crate::index_struct::Index;
use ragit_error::ApiError;
use std::path::PathBuf;
use ragit_types::RemoveResult;

pub async fn index_remove(
    index: &mut Index,
    _path: PathBuf,
    _dry_run: bool,
    _recursive: bool,
    _auto: bool,
    _staged: bool,
    _processed: bool,
) -> Result<RemoveResult, ApiError> {
    index.remove().await
}
