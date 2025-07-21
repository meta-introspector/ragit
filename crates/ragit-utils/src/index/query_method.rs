use crate::error::Error;
use crate::index::index_struct::Index;
use ragit_pdl::Schema;

use crate::query::query_response::QueryResponse;
use crate::query::query_turn::QueryTurn;
use crate::query::select_turns_for_context::select_turns_for_context;

impl Index {
    pub async fn query(
        &self,
        q: &str,
        history: Vec<QueryTurn>,
        schema: Option<Schema>,
    ) -> Result<QueryResponse, Error> {
        // There's no need to rephrase the query if the rag pipeline is disabled.
        let (multi_turn_schema, rephrased_query) = if history.is_empty() || !self.query_config.enable_rag || self.chunk_count == 0 {
            (None, q.to_string())
        } else {
            let multi_turn_schema = self.rephrase_multi_turn(
                select_turns_for_context(&history, q),
            ).await?;
            let rephrased_query = if multi_turn_schema.is_query && multi_turn_schema.in_context {
                multi_turn_schema.query.clone()
            } else {
                q.to_string()
            };

            (Some(multi_turn_schema), rephrased_query)
        };
        let chunks = self.retrieve_chunks(&rephrased_query, self.query_config.super_rerank).await?;

        let response = if chunks.is_empty() {
            let mut history_turns = Vec::with_capacity(history.len() * 2);

            for h in history.iter() {
                history_turns.push(h.query.clone());
                history_turns.push(h.response.response.clone());
            }

            self.raw_request(
                q,
                history_turns,
                schema,
            ).await?
        } else {
            self.answer_query_with_chunks(
                &rephrased_query,
                chunks.clone(),
                schema,
            ).await?
        };

        Ok(QueryResponse {
            multi_turn_schema,
            retrieved_chunks: chunks,
            response,
        })
    }
}
