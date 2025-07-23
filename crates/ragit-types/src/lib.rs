pub mod image;
pub mod chunk;
pub mod uid;

use serde::{Deserialize, Serialize};
//use sha3::{Digest, Sha3_256};
//use std::fmt;
pub use crate::uid::Uid;
pub use crate::image::ImageSchema;
pub use crate::chunk::chunk_struct::{Chunk, ChunkBuildInfo};
//use std::str::FromStr;


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

