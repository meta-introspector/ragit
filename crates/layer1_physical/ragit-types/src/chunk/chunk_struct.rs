use crate::chunk::chunk_source::ChunkSource;
use crate::uid::Uid;
use serde::{Deserialize, Serialize};
//use std::collections::HashMap;

use crate::processed_doc::ProcessedDoc;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ChunkBuildInfo {
    pub file_reader_key: String,
    pub prompt_hash: String,
    pub model: String,
}

impl ChunkBuildInfo {
    pub fn new(file_reader_key: String, prompt_hash: String, model: String) -> Self {
        ChunkBuildInfo {
            file_reader_key,
            prompt_hash,
            model,
        }
    }
}

impl Default for ChunkBuildInfo {
    fn default() -> Self {
        ChunkBuildInfo {
            file_reader_key: String::new(),
            prompt_hash: String::new(),
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
    // pub muse_summaries: HashMap<String, String>,
    pub file: String,
    pub index: usize,
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
//            muse_summaries: HashMap::new(),
            file: String::new(),
            index: 0,
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
//            muse_summaries: HashMap::new(),
            file: String::new(),
            index: 0,
            source: ChunkSource::default(),
            build_info: ChunkBuildInfo::default(),
            timestamp: 0,
            searchable: false,
        }
    }
}

use super::chunk_trait::ChunkLike;

impl ChunkLike for Chunk {
    fn uid(&self) -> &Uid {
        &self.uid
    }

    fn char_len(&self) -> usize {
        self.char_len
    }

    fn image_count(&self) -> usize {
        self.image_count
    }

    fn render_source(&self) -> String {
        self.render_source()
    }
}
