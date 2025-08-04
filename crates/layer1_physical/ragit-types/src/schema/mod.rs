use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Schema(pub String);

pub fn integer_between(min: Option<i128>, max: Option<i128>) -> Schema {
    let mut schema_json = serde_json::json!({
        "type": "integer"
    });

    if let Some(min_val) = min {
        schema_json["minimum"] = serde_json::to_value(min_val).unwrap();
    }
    if let Some(max_val) = max {
        schema_json["maximum"] = serde_json::to_value(max_val).unwrap();
    }

    Schema(schema_json.to_string())
}

pub fn default_yesno() -> Schema {
    Schema(serde_json::json!({
        "type": "boolean",
        "description": "Respond with true for yes, false for no."
    }).to_string())
}

impl Schema {
    pub fn validate(&self, response: &str) -> Result<Value, String> {
        // TODO: Implement actual schema validation logic here
        // For now, just try to parse it as JSON
        serde_json::from_str(response)
            .map_err(|e| format!("Failed to parse response as JSON: {}", e))
    }
}