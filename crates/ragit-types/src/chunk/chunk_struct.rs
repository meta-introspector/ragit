use crate::chunk::chunk_source::ChunkSource;
use crate::uid::Uid;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::processed_doc::ProcessedDoc;
use crate::build_config::BuildConfig;
use crate::chunk::atomic_token::AtomicToken;
use ragit_config::api_config::ApiConfig;
use ragit_error::ApiError as Error;
use regex::Regex;
use serde_json::Value as JsonValue;
use sha3::{Digest, Sha3_256};
use ragit_api::{ChatRequest, Message, MessageContent, RecordAt, Role, messages_from_pdl};
use ragit_fs::{normalize};

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
    pub muse_summaries: HashMap<String, String>,
    pub source: ChunkSource,
    pub uid: Uid,
    pub build_info: ChunkBuildInfo,
    pub timestamp: i64,
    pub searchable: bool,
}
