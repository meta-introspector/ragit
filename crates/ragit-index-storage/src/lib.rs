use std::path::PathBuf;
use ragit_utils::constant::{CHUNK_DIR_NAME, FILE_INDEX_DIR_NAME, IMAGE_DIR_NAME, INDEX_DIR_NAME};
use ragit_error::ApiError;
use ragit_utils::ragit_path_utils::join3_paths;
use ragit_fs::{extension, is_dir, read_dir};

pub fn get_files_from_index_subdir(
    root_dir: &PathBuf,
    subdir_name: &str,
    extension_filter: Option<&str>,
) -> Result<Vec<PathBuf>, ApiError> {
    let mut result = vec![];
    let base_path = join3_paths(
        root_dir,
        &PathBuf::from(INDEX_DIR_NAME),
        &PathBuf::from(subdir_name),
    )?;

    for internal in read_dir(base_path.to_str().unwrap(), false)? {
        if !is_dir(&internal) {
            continue;
        }

        for file_path in read_dir(&internal, false)? {
            if let Some(ext_filter) = extension_filter {
                if let Ok(Some(ext)) = extension(&file_path) {
                    if ext == ext_filter {
                        result.push(file_path.into());
                    }
                }
            } else {
                result.push(file_path.into());
            }
        }
    }

    result.sort();
    Ok(result)
}

pub fn get_all_chunk_files(root_dir: &PathBuf) -> Result<Vec<PathBuf>, ApiError> {
    get_files_from_index_subdir(root_dir, CHUNK_DIR_NAME, Some("chunk"))
}

pub fn get_all_tfidf_files(root_dir: &PathBuf) -> Result<Vec<PathBuf>, ApiError> {
    get_files_from_index_subdir(root_dir, CHUNK_DIR_NAME, Some("tfidf"))
}

pub fn get_all_image_files(root_dir: &PathBuf) -> Result<Vec<PathBuf>, ApiError> {
    get_files_from_index_subdir(root_dir, IMAGE_DIR_NAME, Some("png"))
}

pub fn get_all_file_indexes(root_dir: &PathBuf) -> Result<Vec<PathBuf>, ApiError> {
    get_files_from_index_subdir(root_dir, FILE_INDEX_DIR_NAME, None)
}