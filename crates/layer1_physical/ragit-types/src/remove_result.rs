use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RemoveResult {
    pub removed_files: usize,
    pub removed_chunks: usize,
}