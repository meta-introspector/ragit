use crate::index_struct::Index;
use ragit_types::ApiError;
use std::path::PathBuf;

pub fn index_get_data_path(
    _index: &Index,
    root_dir: &PathBuf,
    file: &PathBuf,
) -> Result<PathBuf, ApiError> {
    println!("index_get_data_path: root_dir: {:?}", root_dir);
    println!("index_get_data_path: file: {:?}", file);
    let full_path = root_dir.join(file);
    println!("index_get_data_path: full_path: {:?}", full_path);
    Ok(full_path)
}
