use crate::prelude::*;

pub fn get_chunk_by_uid(index: &Index, uid: Uid) -> Result<Chunk, ApiError> {
    let chunk_at = get_uid_path(
        &index.root_dir,
        CHUNK_DIR_NAME,
        uid,
        Some("chunk"),
    )?;

    if exists(&chunk_at) {
        return Ok(load_uid_from_file(&chunk_at.to_string_lossy().into_owned())?);
    }

    Err(ApiError::NoSuchChunk(uid.to_string()))
}
