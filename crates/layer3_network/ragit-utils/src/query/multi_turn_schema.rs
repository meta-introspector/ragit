use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct MultiTurnSchema {
    pub is_query: bool,
    pub in_context: bool,
    pub query: String,
}

impl Default for MultiTurnSchema {
    fn default() -> Self {
        MultiTurnSchema {
            is_query: false,
            in_context: false,
            query: String::new(),
        }
    }
}
