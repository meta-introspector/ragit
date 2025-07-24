use ragit_index::Index;
use ragit_index_io::helpers::load_chunk_from_pathbuf;
use std::path::Path;
use ragit_utils::ragit_path_utils::get_uid_path;
use ragit_fs::exists;
use ragit_error::ApiError;
use ragit_types::uid::Uid;
use ragit_types::chunk::chunk_struct::Chunk;
use ragit_utils::constant::CHUNK_DIR_NAME;

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
