use std::path::PathBuf;
use ragit_utils::constant::INDEX_DIR_NAME;
use ragit_types::ApiError;
use ragit_fs::{extension, is_dir, read_dir, join3};

pub fn get_files_from_index_subdir(
    root_dir: &PathBuf,
    subdir_name: &str,
    extension_filter: Option<&str>,
) -> Result<Vec<PathBuf>, ApiError> {
    let mut result = vec![];
    let base_path_str = join3(
        root_dir.to_str().unwrap(),
        INDEX_DIR_NAME,
        subdir_name,
    ).map_err(|e| ApiError::Internal(format!("Failed to join paths: {}", e)))?;

    for internal in read_dir(base_path_str.as_str(), false)? {
        if !is_dir(internal.as_str()) {
            continue;
        }

        for file_path in read_dir(internal.as_str(), false)? {
            if let Some(ext_filter) = extension_filter {
                if let Ok(Some(ext)) = extension(file_path.as_str()) {
                    if ext == ext_filter {
                        result.push(PathBuf::from(file_path));
                    }
                }
            } else {
                result.push(PathBuf::from(file_path));
            }
        }
    }

    result.sort();
    Ok(result)
}