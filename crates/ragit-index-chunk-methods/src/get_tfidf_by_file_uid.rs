use crate::prelude::*;
use std::path::Path;
use crate::get_chunks_of_file;
use crate::get_tfidf_by_chunk_uid;

pub fn get_tfidf_by_file_uid(
    index: &Index,
    uid: Uid,
) -> Result<ProcessedDoc, ApiError> {
    let chunk_uids = crate::get_chunks_of_file::get_chunks_of_file(index, uid)?;
    let mut result = ProcessedDoc::empty();

    for uid in chunk_uids.iter() {
        result.extend(&crate::get_tfidf_by_chunk_uid::get_tfidf_by_chunk_uid(index, *uid)?);
    }

    result.doc_id = uid;
    Ok(result)
}
