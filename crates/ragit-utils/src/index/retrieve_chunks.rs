use crate::chunk::Chunk;
use crate::error::Error;
use crate::index::index_struct::Index;
use tokio::task::JoinSet;


impl Index {
    /// It retrieves chunks that are related to `query`. If `super_rerank` is set, it calls `summaries_to_chunks` multiple times.
    /// That takes longer time, but is likely to have a better result.
    pub async fn retrieve_chunks(&self, query: &str, super_rerank: bool) -> Result<Vec<Chunk>, Error> {
        if !self.query_config.enable_rag || self.chunk_count == 0 {
            return Ok(vec![]);
        }

        let max_summaries = self.query_config.max_summaries;
        let max_retrieval = self.query_config.max_retrieval;
        let tfidf_limit = if super_rerank { max_summaries * 4 } else { max_summaries };
        let mut chunks = self.load_chunks_or_tfidf(query, tfidf_limit).await?;

        // Let's say `max_summaries` is 10.
        // If `chunks.len()` is 41, it reranks 5 times (9, 9, 9, 9, 5)
        // If `chunks.len()` is 40, it reranks 4 times (10, 10, 10, 10).
        // If `chunks.len()` is 39, it reranks 4 times (10, 10, 10, 9).
        while chunks.len() > max_summaries {  // when `super_rerank` is set
            let mut join_set = JoinSet::new();
            let mut new_chunks = vec![];

            let mut slide_size = max_summaries;

            if chunks.len() % slide_size < slide_size / 2 {
                slide_size -= 1;
            }

            for cc in chunks.chunks(slide_size) {
                let index = self.clone();
                let query = query.to_string();
                let cc = cc.to_vec();

                join_set.spawn(async move {
                    index.summaries_to_chunks(&query, cc, max_retrieval.max(max_summaries / 2)).await
                });
            }

            while let Some(res) = join_set.join_next().await {
                new_chunks.append(&mut res??);
            }

            chunks = new_chunks;
        }

        if chunks.len() > max_retrieval {
            chunks = self.summaries_to_chunks(
                query,
                chunks,
                max_retrieval,
            ).await?;
        }

        Ok(chunks)
    }
}
