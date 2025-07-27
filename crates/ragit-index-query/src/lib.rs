use ragit_index_types::index_struct::Index;
use ragit_types::ApiError;
use ragit_model_query_response::ModelQueryResponse;
use ragit_types::query_turn::QueryTurn;

pub mod prelude;

pub async fn query(
    index: &Index,
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