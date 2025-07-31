use ragit_model::Model;
use crate::chunk::chunk_struct::Chunk;
use serde::{Deserialize, Serialize};

use ragit_utils::query::MultiTurnSchema;
use ragit_pdl::Prompt;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ModelQueryResponse {
    pub multi_turn_schema: Option<MultiTurnSchema>,
    pub retrieved_chunks: Vec<Chunk>,
    pub response: String,
}

impl ModelQueryResponse {
    pub async fn new(
        _model: &Model,
        _prompt: &Prompt,
        _previous_request: &str,
        _current_data: &str,
    ) -> Result<Self, crate::ApiError> {
        Ok(ModelQueryResponse {
            multi_turn_schema: None,
            retrieved_chunks: Vec::new(),
            response: String::new(),
        })
    }
}
