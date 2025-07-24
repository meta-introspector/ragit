use crate::prelude::*;
use crate::get_chunks_of_file;
use crate::get_chunk_by_uid;

pub fn get_merged_chunk_of_file(index: &Index, file_uid: Uid) -> Result<RenderedChunk, ApiError> {
    let chunk_uids = get_chunks_of_file(index, file_uid)?;
    let mut chunks = Vec::with_capacity(chunk_uids.len());

    for chunk_uid in chunk_uids.iter() {
        chunks.push(get_chunk_by_uid(index, *chunk_uid)?);
    }

    // FIXME: I don't think we have to sort this
    // chunks.sort_by_key(|chunk| chunk.sortable_string()); // Changed sort_by_by_key to sort_by_key
    let chunks = merge_and_convert_chunks(index, chunks)?;

    match chunks.len() {
        0 => todo!(),  // It's an empty file. Does ragit create a chunk for an empty file? I don't remember...
        1 => Ok(chunks[0].clone()),
        _ => Err(ApiError::BrokenIndex(format!("Internal error: `get_merged_chunk_of_file({file_uid})` returned multiple chunks"))),
    }
}

// Shim for merge_and_convert_chunks
pub fn merge_and_convert_chunks(_index: &Index, chunks: Vec<Chunk>) -> Result<Vec<RenderedChunk>, ApiError> {
    // This is a temporary shim. Actual implementation will be moved here later.
    Ok(chunks.into_iter().map(|c| RenderedChunk {
        pdl_data: c.data.clone(),
        human_data: c.data.clone(),
        raw_data: Vec::new(), // Placeholder
        source: c.render_source(),
    }).collect())
}
