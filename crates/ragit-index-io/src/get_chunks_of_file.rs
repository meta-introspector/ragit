
use ragit_index_types::index_struct::Index;
use std::path::PathBuf;
use std::path::Path;
use ragit_utils::ragit_path_utils::get_uid_path;
use ragit_fs::{exists, read_string};
use ragit_error::ApiError;
use ragit_types::uid::Uid;
use ragit_utils::constant::FILE_INDEX_DIR_NAME;

pub fn get_chunks_of_file(index: &Index, file_uid: Uid) -> Result<Vec<Uid>, ApiError> {
    let file_index_path = get_uid_path(
        &index.root_dir,
        Path::new(FILE_INDEX_DIR_NAME),
        file_uid,
        None,
    )?;

    if exists(&file_index_path) {
        let content = read_string(&file_index_path.to_string_lossy().into_owned())?;
        let uids: Vec<Uid> = serde_json::from_str(&content)?;
        return Ok(uids);
    }

    Err(ApiError::NoSuchFile { path: None, uid: Some(file_uid.to_string()) })
}

fn load_uid_from_file(path: &PathBuf) -> Result<Vec<Uid>, ApiError> {
    let content = read_string(&path.to_string_lossy().into_owned())?;
    Ok(serde_json::from_str(&content)?)
}
