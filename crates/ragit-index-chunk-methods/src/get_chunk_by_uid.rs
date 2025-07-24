use crate::prelude::*;
use crate::helpers::load_chunk_from_pathbuf;
use std::path::Path;

pub fn get_chunk_by_uid(index: &Index, uid: Uid) -> Result<Chunk, ApiError> {
    let chunk_at = get_uid_path(
        &index.root_dir,
        Path::new(CHUNK_DIR_NAME),
        uid,
        Some("chunk"),
    )?;

    if exists(&chunk_at) {
        return Ok(load_chunk_from_pathbuf(&chunk_at)?);
    }

    Err(ApiError::NoSuchChunk(uid.to_string()))
}
