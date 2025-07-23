use crate::chunk::Chunk;
use crate::prelude::*;

use crate::index::index_struct::Index;

impl Index {
    pub(crate) async fn extract_keywords(&self, query: &str) -> Result<Keywords> {
        Err(anyhow!("Not implemented"))
    }

    pub(crate) async fn load_chunks_or_tfidf(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<Chunk>, Error> {
        if self.chunk_count > limit {
            let keywords = self.extract_keywords(query).await?;
            let tfidf_results = self.run_tfidf(keywords, limit)?;
            let mut chunks = Vec::with_capacity(tfidf_results.len());

            for tfidf_result in tfidf_results.into_iter() {
                let uid = tfidf_result.id;
                chunks.push(self.get_chunk_by_uid(uid)?);
            }

            Ok(chunks)
        } else {
            let mut chunks = vec![];

            for chunk_path in &self.get_all_chunk_files()? {
                chunks.push(crate::chunk::load_from_file(chunk_path)?);
            }

            Ok(chunks)
        }
    }
}
