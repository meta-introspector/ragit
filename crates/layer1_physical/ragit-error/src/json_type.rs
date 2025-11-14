use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JsonType {
    Null,
    Bool,
    Number,
    String,
    Array,
    Object,
    U64,
}

impl From<&serde_json::Value> for JsonType {
    fn from(value: &serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => JsonType::Null,
            serde_json::Value::Bool(_) => JsonType::Bool,
            serde_json::Value::Number(_) => JsonType::Number,
            serde_json::Value::String(_) => JsonType::String,
            serde_json::Value::Array(_) => JsonType::Array,
            serde_json::Value::Object(_) => JsonType::Object,
        }
    }
}

impl std::fmt::Display for JsonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
