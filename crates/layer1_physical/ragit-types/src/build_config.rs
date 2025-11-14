use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

impl Default for BuildConfig {
    fn default() -> Self {
        BuildConfig {
            chunk_size: 256,
            image_size: 1024,
            slide_len: 512,
            compression_threshold: 1024,
            compression_level: 9,
            min_summary_len: 50,
            max_summary_len: 200,
            summary_after_build: false,
            strict_file_reader: false,
        }
    }
}
