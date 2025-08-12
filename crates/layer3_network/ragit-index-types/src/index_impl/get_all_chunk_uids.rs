use crate::index_struct::Index;
use ragit_types::ApiError;
use ragit_types::uid::Uid;

impl Index {
    pub fn get_all_chunk_uids(&self) -> Result<Vec<Uid>, ApiError> {
        eprintln!("Placeholder for get_all_chunk_uids");
        Ok(Vec::new())
    }
}