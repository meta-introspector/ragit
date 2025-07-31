use serde::{Deserialize, Serialize};
use crate::uid::Uid;

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
