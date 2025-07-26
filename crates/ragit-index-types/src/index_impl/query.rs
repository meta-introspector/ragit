use crate::index_struct::Index;
use ragit_error::ApiError;
use ragit_model_query_response::ModelQueryResponse;
use ragit_types::query_turn::QueryTurn;

impl Index {
    pub async fn query(
        &self,
        query: &str,
        turns: Vec<QueryTurn>,
        model_override: Option<String>,
    ) -> Result<ModelQueryResponse, ApiError> {
        eprintln!(
            "Placeholder for query: query={}, turns={:?}, model_override={:?}",
            query, turns, model_override
        );
        Ok(ModelQueryResponse::default())
    }
}