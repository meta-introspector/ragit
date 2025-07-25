use serde::{Deserialize, Serialize};
use ragit_index_core::{Index, LoadMode};
use ragit_error::ApiError;
use ragit_types::chunk::chunk_struct::Chunk;
use ragit_tfidf::TfidfResult;
use tokio::task::JoinSet;
use ragit_index_io::get_chunk_by_uid::get_chunk_by_uid;

pub mod prelude;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueryResponse {
    pub response: String,
    // Add other fields as needed based on original definition
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueryTurn {
    pub query: String,
    pub response: QueryResponse,
}

impl Index {
    pub async fn retrieve_chunks(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<Chunk>, ApiError> {
        // Assuming run_tfidf will be moved to ragit-tfidf or ragit-index-tfidf
        // For now, we'll use a placeholder.
        // TODO: Call run_tfidf from ragit-tfidf or ragit-index-tfidf
        let tfidf_results: Vec<TfidfResult> = Vec::new(); // Placeholder
        let mut chunks = Vec::with_capacity(tfidf_results.len());
        let mut join_set = JoinSet::new();

        for tfidf_result in tfidf_results {
            let index_clone = self.clone();
            join_set.spawn(async move {
                get_chunk_by_uid(&index_clone, tfidf_result.doc_id)
            });
        }

        while let Some(res) = join_set.join_next().await {
            chunks.push(res.unwrap()?);
        }

        Ok(chunks)
    }
}