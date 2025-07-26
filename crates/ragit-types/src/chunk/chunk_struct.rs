use crate::chunk::chunk_source::ChunkSource;
use crate::uid::Uid;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::processed_doc::ProcessedDoc;

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

impl Chunk {
    pub fn render_source(&self) -> String {
        // Implement your rendering logic here based on ChunkSource
        // For now, a placeholder implementation
        format!("Source: {:?}", self.source)
    }

    pub fn dummy() -> Self {
        Self {
            data: String::new(),
            images: Vec::new(),
            char_len: 0,
            image_count: 0,
            title: String::new(),
            summary: String::new(),
            muse_summaries: HashMap::new(),
            source: ChunkSource::default(),
            uid: Uid::dummy(),
            build_info: ChunkBuildInfo::default(),
            timestamp: 0,
            searchable: false,
        }
    }
}

impl From<ProcessedDoc> for Chunk {
    fn from(doc: ProcessedDoc) -> Self {
        Chunk {
            data: doc.tokens.join(" "), // Join tokens to form data
            uid: doc.doc_id,
            // Set other fields to default or empty values
            images: Vec::new(),
            char_len: 0,
            image_count: 0,
            title: String::new(),
            summary: String::new(),
            muse_summaries: HashMap::new(),
            source: ChunkSource::default(),
            build_info: ChunkBuildInfo::default(),
            timestamp: 0,
            searchable: false,
        }
    }
}
