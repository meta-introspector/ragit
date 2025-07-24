use crate::prelude::*;

pub fn get_tfidf_by_chunk_uid(
    index: &Index,
    uid: Uid,
) -> Result<ProcessedDoc, ApiError> {
    let tfidf_at = get_uid_path(
        &index.root_dir,
        Path::new(CHUNK_DIR_NAME),
        uid,
        Some("tfidf"),
    )?;

    if exists(&tfidf_at) {
        return Ok(load_tfidf_from_file(&tfidf_at.to_string_lossy().into_owned())?);
    }

    Err(ApiError::NoSuchChunk(uid.to_string()))
}
