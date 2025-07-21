use crate::chunk::Chunk;
use serde::{Deserialize, Serialize};

use super::MultiTurnSchema;
use ragit_api::Model;
use ragit_pdl::Prompt;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueryResponse {
    pub multi_turn_schema: Option<MultiTurnSchema>,
    pub retrieved_chunks: Vec<Chunk>,
    pub response: String,
}

impl QueryResponse {
    pub async fn new(
        _model: &Model,
        _prompt: &Prompt,
        _previous_request: &str,
        _current_data: &str,
    ) -> Result<Self, crate::prelude::Error> {
        Ok(QueryResponse {
            multi_turn_schema: None,
            retrieved_chunks: Vec::new(),
            response: String::new(),
        })
    }
}
