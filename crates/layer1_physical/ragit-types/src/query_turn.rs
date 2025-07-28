use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct QueryResponse {
    pub response: String,
    // Add other fields as needed based on original definition
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueryTurn {
    pub query: String,
    pub response: QueryResponse,
}
