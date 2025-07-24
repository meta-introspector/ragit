pub mod image;
pub mod chunk;
pub mod uid;
pub mod prelude;
pub mod pdl_types;
pub mod api_error;
pub use api_error::ApiError;

use serde::{Deserialize, Serialize};
pub use crate::uid::Uid;
pub use crate::image::ImageSchema;
pub use crate::chunk::chunk_struct::{Chunk, ChunkBuildInfo};
pub mod response;
pub mod test_model;
pub use test_model::TestModel;

//pub use crate::pdl_types::{ImageType, JsonType, Message, MessageContent, PdlRole, Role};

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

#[derive(Clone, Debug)]
pub struct AuditRecordAt {
    pub path: String,
    pub id: String,
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileSchema {
    pub path: String,
    pub is_processed: bool,
    pub length: u64,
    pub uid: Uid,
    pub chunks: usize,
    pub model: String,
    pub last_updated: u64,
}

impl FileSchema {
    pub fn dummy() -> Self {
        FileSchema {
            path: String::new(),
            is_processed: false,
            length: 0,
            uid: Uid::dummy(),
            chunks: 0,
            model: String::new(),
            last_updated: 0,
        }
    }
}

