use anyhow::anyhow;
use crate::prelude::*;

impl Index {
    pub async fn extract_keywords(&self, _query: &str) -> Result<Keywords, ApiError> {
        Err(anyhow!("Not implemented"))
    }

    pub async fn load_chunks_from_uids(
        &self,
        uids: &[Uid],
    ) -> Result<Vec<Chunk>, ApiError> {
        let mut chunks = Vec::with_capacity(uids.len());

        for uid in uids {
            chunks.push(self.get_chunk_by_uid(*uid)?);
        }

        Ok(chunks)
    }

    pub async fn load_all_chunks(&self) -> Result<Vec<Chunk>, ApiError> {
        let mut chunks = vec![];

        for chunk_path in &self.get_all_chunk_files()? {
            chunks.push(ragit_types::chunk::load_from_file(chunk_path)?);
        }

        Ok(chunks)
    }
}