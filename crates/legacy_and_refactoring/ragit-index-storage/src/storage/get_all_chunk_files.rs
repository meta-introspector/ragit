use ragit_types::ApiError;
use ragit_index_types::index_struct::Index;
use ragit_types::fixed_types::fixed_chunk_struct::FixedChunk;

pub fn get_all_chunk_files(index: &Index) -> Result<Vec<FixedChunk>, ApiError> {
    Ok(index.chunks.clone())
}
