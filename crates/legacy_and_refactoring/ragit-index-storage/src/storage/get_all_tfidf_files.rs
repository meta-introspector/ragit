use std::path::PathBuf;
use ragit_types::ApiError;
use ragit_utils::constant::CHUNK_DIR_NAME;
use crate::storage::get_files_from_index_subdir::get_files_from_index_subdir;

pub fn get_all_tfidf_files(root_dir: &PathBuf) -> Result<Vec<PathBuf>, ApiError> {
    get_files_from_index_subdir(root_dir, CHUNK_DIR_NAME, Some("tfidf"))
}
