use crate::chunk::chunk_struct::Chunk;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct ChunkSchema {
    pub title: String,
    pub summary: String,
}

impl ChunkSchema {
    pub fn dummy(_data: &str, _len: usize) -> Self {
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
