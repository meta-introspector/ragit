use crate::index_struct::Index;
use ragit_error::ApiError;
use ragit_types::uid::Uid;
use ragit_types::chunk::chunk_struct::Chunk;

impl Index {
    pub fn get_chunk_by_uid(&self, chunk_uid: Uid) -> Result<Chunk, ApiError> {
        eprintln!("Placeholder for get_chunk_by_uid: chunk_uid={}", chunk_uid);
        Err(ApiError::BrokenIndex("Placeholder for get_chunk_by_uid".to_string()))
    }
}