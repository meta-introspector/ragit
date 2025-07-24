use crate::prelude::*;
use ragit_index_types::Index;
use ragit_tfidf::load_from_file;
impl Index {
    pub async fn extract_keywords(&self, _query: &str) -> Result<Keywords, ApiError> {
        Err(ApiError::NotImplemented("extract_keywords"))
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
            chunks.push(load_from_file(chunk_path)?);
        }

        Ok(chunks)
    }
}
