use serde::{Deserialize, Serialize};
use crate::ModelQueryResponse;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueryTurn {
    pub query: String,
    pub response: ModelQueryResponse,
}

impl QueryTurn {
    pub fn new(query: String, response: ModelQueryResponse) -> Self {
        QueryTurn { query, response }
    }
}
