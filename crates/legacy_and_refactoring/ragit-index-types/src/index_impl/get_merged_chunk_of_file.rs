use crate::index_struct::Index;
use ragit_types::ApiError;
use ragit_types::uid::Uid;
use ragit_types::chunk::rendered_chunk::RenderedChunk;

impl Index {
    pub fn get_merged_chunk_of_file(
        &self,
        file_uid: Uid,
    ) -> Result<RenderedChunk, ApiError> {
        eprintln!(
            "Placeholder for get_merged_chunk_of_file: file_uid={}",
            file_uid
        );
        Err(ApiError::BrokenIndex("Placeholder for get_merged_chunk_of_file".to_string()))
    }
}