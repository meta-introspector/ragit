use crate::prelude::*;
use crate::get_chunks_of_file;
use crate::get_tfidf_by_chunk_uid;

pub fn get_tfidf_by_file_uid(
    index: &Index,
    uid: Uid,
) -> Result<ProcessedDoc, ApiError> {
    let chunk_uids = get_chunks_of_file(index, uid)?;
    let mut result = ProcessedDoc::empty();

    for uid in chunk_uids.iter() {
        result.extend(&get_tfidf_by_chunk_uid(index, *uid)?);
    }

    result.doc_id = uid;
    Ok(result)
}
