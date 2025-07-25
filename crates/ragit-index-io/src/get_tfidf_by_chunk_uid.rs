
use ragit_index_types::Index;
use std::path::Path;
use ragit_utils::path_utils::get_uid_path;
use ragit_fs::exists;
use ragit_error::ApiError;
use ragit_types::uid::Uid;
use ragit_tfidf::{ProcessedDoc, load_from_file as load_tfidf_from_file};
use ragit_utils::constant::CHUNK_DIR_NAME;

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