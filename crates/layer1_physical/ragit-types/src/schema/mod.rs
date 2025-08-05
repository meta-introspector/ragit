use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Schema(pub String);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SchemaType {
    String,
    Integer,
    Float,
    Boolean,
    YesNo,
    Code,
    TaskList,
    Array(Box<SchemaType>),
    Object(std::collections::HashMap<String, SchemaType>),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Constraint {
    Minimum(i128),
    Maximum(i128),
    Pattern(String),
    Enum(Vec<String>),
    MinLength(usize),
    MaxLength(usize),
    MinItems(usize),
    MaxItems(usize),
    UniqueItems(bool),
    Required(Vec<String>),
}

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

pub fn default_string() -> Schema {
    Schema(serde_json::json!({
        "type": "string"
    }).to_string())
}

pub fn default_integer() -> Schema {
    Schema(serde_json::json!({
        "type": "integer"
    }).to_string())
}

pub fn default_float() -> Schema {
    Schema(serde_json::json!({
        "type": "number"
    }).to_string())
}

pub fn default_boolean() -> Schema {
    Schema(serde_json::json!({
        "type": "boolean"
    }).to_string())
}

pub fn default_code() -> Schema {
    Schema(serde_json::json!({
        "type": "string",
        "format": "code"
    }).to_string())
}

pub fn default_task_list() -> Schema {
    Schema(serde_json::json!({
        "type": "array",
        "items": {
            "type": "object",
            "properties": {
                "task": {
                    "type": "string"
                },
                "completed": {
                    "type": "boolean"
                }
            },
            "required": ["task", "completed"]
        }
    }).to_string())
}

pub fn default_array(items_schema: Schema) -> Schema {
    Schema(serde_json::json!({
        "type": "array",
        "items": serde_json::from_str::<Value>(&items_schema.0).unwrap()
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