use ragit_types::chunk::chunk_struct::Chunk;
use serde::{Deserialize, Serialize};

use ragit_utils::query::MultiTurnSchema;

pub mod query_turn;
pub use query_turn::QueryTurn;
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

    pub fn new(response: String) -> Self {
        ModelQueryResponse {
            multi_turn_schema: None,
            retrieved_chunks: Vec::new(),
            response,
        }
    }
}
