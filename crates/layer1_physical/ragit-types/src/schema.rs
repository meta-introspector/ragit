use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Schema(pub String);

impl Schema {
    pub fn validate(&self, response: &str) -> Result<Value, String> {
        // TODO: Implement actual schema validation logic here
        // For now, just try to parse it as JSON
        serde_json::from_str(response)
            .map_err(|e| format!("Failed to parse response as JSON: {}", e))
    }
}
