use crate::prelude::*;

pub fn get_chunks_of_file(index: &Index, file_uid: Uid) -> Result<Vec<Uid>, ApiError> {
    let file_index_path = get_uid_path(
        &index.root_dir,
        Path::new(FILE_INDEX_DIR_NAME),
        file_uid,
        None,
    )?;

    if exists(&file_index_path) {
        return Ok(load_uid_from_file(&file_index_path.to_string_lossy().into_owned())?);
    }

    Err(ApiError::NoSuchFile { path: None, uid: Some(file_uid.to_string()) })
}
