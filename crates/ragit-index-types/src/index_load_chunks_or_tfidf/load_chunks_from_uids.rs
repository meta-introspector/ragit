use crate::prelude::*;
use crate::index_struct::Index;
use ragit_error::ApiError;
use ragit_types::{Uid, Chunk};

impl Index {
    pub async fn load_chunks_from_uids(
        &self,
        uids: &[Uid],
    ) -> Result<Vec<Chunk>, ApiError> {
        let mut chunks = Vec::with_capacity(uids.len());

        for uid in uids {
            chunks.push(ragit_index_chunk_methods::get_chunk_by_uid::get_chunk_by_uid(self, *uid)?);
        }

        Ok(chunks)
    }
}
