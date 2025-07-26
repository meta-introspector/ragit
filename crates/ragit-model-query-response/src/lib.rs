use ragit_model::Model;
use ragit_types::chunk::chunk_struct::Chunk;
use serde::{Deserialize, Serialize};
use ragit_error::ApiError as Error;
use ragit_utils::query::MultiTurnSchema;
pub use ragit_pdl::Prompt;
pub mod query_turn;
pub use query_turn::QueryTurn;
use ragit_error::ApiError;
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct ModelQueryResponse {
    pub multi_turn_schema: Option<MultiTurnSchema>,
    pub retrieved_chunks: Vec<Chunk>,
    pub response: String,
}

impl ModelQueryResponse {
    pub fn get_message(&self) -> &str {
        &self.response
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    pub async fn new(
        _model: &Model,
        _prompt: &Prompt,
        _previous_request: &str,
        _current_data: &str,
    ) -> Result<Self, ApiError> {
        Ok(ModelQueryResponse {
            multi_turn_schema: None,
            retrieved_chunks: Vec::new(),
            response: String::new(),
        })
    }
}
