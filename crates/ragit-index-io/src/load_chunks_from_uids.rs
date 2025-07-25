
use ragit_index_types::Index;
use ragit_error::ApiError;
use ragit_types::{Uid, Chunk};
use crate::get_chunk_by_uid::get_chunk_by_uid;

pub async fn load_chunks_from_uids(
    index: &Index,
    uids: &[Uid],
) -> Result<Vec<Chunk>, ApiError> {
    let mut chunks = Vec::with_capacity(uids.len());

    for uid in uids {
        chunks.push(get_chunk_by_uid(index, *uid)?);
    }

    Ok(chunks)
}