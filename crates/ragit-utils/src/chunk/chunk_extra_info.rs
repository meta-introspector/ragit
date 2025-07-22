use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChunkExtraInfo {
    pub page_no: Option<usize>,
}

impl ChunkExtraInfo {
    pub fn merge(&self, other: &ChunkExtraInfo) -> ChunkExtraInfo {
        Self { page_no: None }
    }
}
