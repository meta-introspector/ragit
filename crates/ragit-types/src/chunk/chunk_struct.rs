use crate::chunk::chunk_source::ChunkSource;
use crate::uid::Uid;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ChunkBuildInfo {
    pub model: String,
}

impl Default for ChunkBuildInfo {
    fn default() -> Self {
        ChunkBuildInfo {
            model: String::new(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Chunk {
    pub data: String,
    pub images: Vec<Uid>,
    pub char_len: usize,
    pub image_count: usize,
    pub title: String,
    pub summary: String,
    pub muse_summaries: HashMap<String, String>,
    pub source: ChunkSource,
    pub uid: Uid,
    pub build_info: ChunkBuildInfo,
    pub timestamp: i64,
    pub searchable: bool,
}
