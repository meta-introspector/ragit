use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
