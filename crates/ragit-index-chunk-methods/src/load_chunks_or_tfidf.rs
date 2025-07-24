use crate::prelude::*;
use crate::get_all_chunk_files; // Import the standalone function
use crate::helpers; // Import helpers module

pub async fn load_chunks_or_tfidf(
    index: &Index,
    query: &str,
    limit: usize,
) -> Result<Vec<Chunk>, ApiError> {
    let mut chunks = vec![];

    for chunk_path in crate::get_all_chunk_files::get_all_chunk_files(index)?.into_iter() {
        chunks.push(helpers::load_chunk_from_pathbuf(&chunk_path)?);
    }

    Ok(chunks)
}
