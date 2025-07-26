use crate::index_struct::Index;
use ragit_error::ApiError;
use std::path::PathBuf;

pub fn index_get_data_path(
    index: &Index,
    root_dir: &PathBuf,
    file: &PathBuf,
) -> Result<PathBuf, ApiError> {
    index.get_data_path(root_dir, file)
}
