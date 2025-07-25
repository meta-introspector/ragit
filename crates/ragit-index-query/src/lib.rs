use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod prelude;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueryResponse {
    pub response: String,
    // Add other fields as needed based on original definition
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueryTurn {
    pub query: String,
    pub response: QueryResponse,
}
