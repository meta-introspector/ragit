use crate::index_struct::Index;
use ragit_types::ApiError;
use std::path::PathBuf;

pub fn index_get_data_path(
    _index: &Index,
    _root_dir: &PathBuf,
    _file: &PathBuf,
) -> Result<PathBuf, ApiError> {
    Ok(PathBuf::new())
}
