use std::path::PathBuf;
use ragit_types::ApiError;
use ragit_utils::constant::FILE_INDEX_DIR_NAME;
use crate::storage::get_files_from_index_subdir::get_files_from_index_subdir;

pub fn get_all_file_indexes(root_dir: &PathBuf) -> Result<Vec<PathBuf>, ApiError> {
    get_files_from_index_subdir(root_dir, FILE_INDEX_DIR_NAME, None)
}
