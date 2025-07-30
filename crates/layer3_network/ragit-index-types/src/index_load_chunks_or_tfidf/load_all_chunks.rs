use crate::prelude::*;
use crate::index_struct::Index;
use ragit_tfidf::load_from_file;
use ragit_types::ApiError;
use ragit_types::Chunk;

impl Index {
    pub async fn load_all_chunks(&self) -> Result<Vec<Chunk>, ApiError> {
        let mut chunks = vec![];

        for chunk_path in &ragit_index_chunk_methods::file_retrieval_methods::get_all_chunk_files(self)? {
            // Assuming load_from_file returns a Chunk, not ProcessedDoc
            chunks.push(ragit_tfidf::io::load_from_file(chunk_path.to_str().unwrap())?);
        }

        Ok(chunks)
    }
}
