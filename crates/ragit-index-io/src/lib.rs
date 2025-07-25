pub mod helpers;
pub mod get_tfidf_by_chunk_uid;
pub mod get_chunk_by_uid;
pub mod get_chunks_of_file;
pub mod summaries_to_chunks;
pub mod load_all_chunks;
pub mod load_chunks_from_uids;
pub mod extract_keywords;
pub mod retrieve_chunks;
pub mod file_retrieval_methods;

use ragit_index_core::Index;
use ragit_error::ApiError;
use std::path::PathBuf;

pub fn load_index_from_path(path: &PathBuf) -> Result<Index, ApiError> {
    let index_json = std::fs::read_to_string(path)?;
    let index: Index = serde_json::from_str(&index_json)?;
    Ok(index)
}