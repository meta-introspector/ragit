use crate::chunk::Chunk;
use serde::{Deserialize, Serialize};

use super::MultiTurnSchema;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueryResponse {
    pub multi_turn_schema: Option<MultiTurnSchema>,
    pub retrieved_chunks: Vec<Chunk>,
    pub response: String,
}
