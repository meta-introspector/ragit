use crate::prelude::*;

pub fn get_tfidf_by_file_uid(
    index: &Index,
    uid: Uid,
) -> Result<ProcessedDoc, ApiError> {
    let chunk_uids = index.get_chunks_of_file(uid)?;
    let mut result = ProcessedDoc::empty();

    for uid in chunk_uids.iter() {
        result.extend(&index.get_tfidf_by_chunk_uid(*uid)?);
    }

    result.doc_id = uid;
    Ok(result)
}
