pub use crate::prelude::*;

impl Index {
    pub async fn retrieve_chunks(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<Chunk>, ApiError> {
        let tfidf_results = self.run_tfidf(&Keywords::from(vec![query.to_string()]), limit)?;
        let mut chunks = Vec::with_capacity(tfidf_results.len());
        let mut join_set = tokio::task::JoinSet::new();

        for tfidf_result in tfidf_results {
            let index_clone = self.clone();
            join_set.spawn(async move {
                index_clone.get_chunk_by_uid(tfidf_result.doc_id)
            });
        }

        while let Some(res) = join_set.join_next().await {
            chunks.push(res.unwrap()?);
        }

        Ok(chunks)
    }
}
