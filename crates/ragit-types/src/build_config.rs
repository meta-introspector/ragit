use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BuildConfig {
    pub chunk_size: usize,
    pub image_size: usize,
    pub slide_len: usize,
    pub compression_threshold: u64,
    pub compression_level: u32,
    pub min_summary_len: usize,
    pub max_summary_len: usize,
    pub summary_after_build: bool,
    pub strict_file_reader: bool,
}
