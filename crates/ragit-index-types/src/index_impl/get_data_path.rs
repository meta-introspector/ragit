use crate::index_struct::Index;
use ragit_types::ApiError;
use std::path::PathBuf;

pub fn index_get_data_path(
    index: &Index,
    root_dir: &PathBuf,
    file: &PathBuf,
) -> Result<PathBuf, ApiError> {
    Ok(PathBuf::new())
}
