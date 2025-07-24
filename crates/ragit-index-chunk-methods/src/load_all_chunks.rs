use crate::prelude::*;
use ragit_index_types::Index;
use ragit_tfidf::load_from_file;
use ragit_error::ApiError;
use ragit_types::Chunk;

pub async fn load_all_chunks(
    index: &Index,
) -> Result<Vec<Chunk>, ApiError> {
    let mut chunks = vec![];

    for chunk_path in &ragit_index_chunk_methods::file_retrieval_methods::get_all_chunk_files(index)? {
        // Assuming load_from_file returns a Chunk, not ProcessedDoc
        chunks.push(load_from_file(chunk_path)?);
    }

    Ok(chunks)
}
