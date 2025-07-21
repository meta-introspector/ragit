pub mod render;
pub mod render_impl;

use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::prelude::*;
use crate::Uid;

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
pub enum MultiModalContent {
    Text { content: String },
    Image { uid: Uid },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RenderedChunk {
    pub pdl_data: String,
    pub human_data: String,
    pub raw_data: Vec<MultiModalContent>,
    pub source: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ChunkSource {
    File { path: String, index: usize, page: Option<usize> },
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
    pub fn dummy(data: String, source: ChunkSource) -> Self {
        Self {
            images: vec![],
            char_len: data.chars().count(),
            image_count: 0,
            title: String::new(),
            summary: String::new(),
            muse_summaries: HashMap::new(),
            uid: Uid::dummy(),
            timestamp: 0,
            searchable: true,
            build_info: ChunkBuildInfo::default(),
            data,
            source,
        }
    }

    pub fn render_source(&self) -> String {
        String::new()
    }

    pub fn sortable_string(&self) -> String {
        String::new()
    }

    pub(crate) fn get_approx_size(&self) -> usize {
        0
    }
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct ChunkSchema {
    pub title: String,
    pub summary: String,
}

impl ChunkSchema {
    pub fn dummy(data: &str, len: usize) -> Self {
        Self {
            title: String::new(),
            summary: String::new(),
        }
    }

    pub fn empty() -> Self {
        Self {
            title: String::new(),
            summary: String::new(),
        }
    }

    pub fn render(&self) -> String {
        String::new()
    }
}

impl From<&Chunk> for ChunkSchema {
    fn from(chunk: &Chunk) -> ChunkSchema {
        ChunkSchema {
            title: chunk.title.clone(),
            summary: chunk.summary.clone(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChunkExtraInfo {
    pub page_no: Option<usize>,
}

impl ChunkExtraInfo {
    pub fn merge(&self, other: &ChunkExtraInfo) -> ChunkExtraInfo {
        Self { page_no: None }
    }
}

pub fn load_from_file(path: &PathBuf) -> Result<Chunk> {
    Err(anyhow!("Not implemented"))
}

pub fn save_to_file(
    path: &PathBuf,
    chunk: &Chunk,
    compression_threshold: u64,
    compression_level: u32,
    root_dir: &PathBuf,
    create_tfidf: bool,
) -> Result<()> {
    Err(anyhow!("Not implemented"))
}

pub fn merge_and_convert_chunks(index: &crate::index::index_struct::Index, chunks: Vec<RenderedChunk>) -> Result<Vec<RenderedChunk>> {
    Err(anyhow!("Not implemented"))
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum AtomicToken {
    String { data: String, char_len: usize },
    Image(crate::index::file::Image),
    WebImage { subst: String, url: String },
    PageBreak,
    ChunkExtraInfo(ChunkExtraInfo),
}

pub fn escape_pdl_tokens(s: &str) -> String {
    s.to_string()
}

pub fn merge_overlapping_strings(s1: &[u8], s2: &[u8]) -> String {
    String::new()
}

pub fn into_multi_modal_contents(data: &str, images: &[Uid]) -> Vec<MultiModalContent> {
    // Placeholder implementation
    vec![MultiModalContent::Text { content: data.to_string() }]
}