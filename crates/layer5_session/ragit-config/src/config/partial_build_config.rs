use serde::{Deserialize, Serialize};
use crate::config::build_config::BuildConfig;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PartialBuildConfig {
    pub chunk_size: Option<usize>,
    pub slide_len: Option<usize>,
    pub image_size: Option<usize>,
    pub min_summary_len: Option<usize>,
    pub max_summary_len: Option<usize>,
    pub strict_file_reader: Option<bool>,
    pub compression_threshold: Option<u64>,
    pub compression_level: Option<u32>,
    pub summary_after_build: Option<bool>,
}

impl PartialBuildConfig {
    pub fn apply_to(&self, config: &mut BuildConfig) {
        if let Some(chunk_size) = self.chunk_size {
            config.chunk_size = chunk_size;
        }
        if let Some(slide_len) = self.slide_len {
            config.slide_len = slide_len;
        }
        if let Some(image_size) = self.image_size {
            config.image_size = image_size;
        }
        if let Some(min_summary_len) = self.min_summary_len {
            config.min_summary_len = min_summary_len;
        }
        if let Some(max_summary_len) = self.max_summary_len {
            config.max_summary_len = max_summary_len;
        }
        if let Some(strict_file_reader) = self.strict_file_reader {
            config.strict_file_reader = strict_file_reader;
        }
        if let Some(compression_threshold) = self.compression_threshold {
            config.compression_threshold = compression_threshold;
        }
        if let Some(compression_level) = self.compression_level {
            config.compression_level = compression_level;
        }
        if let Some(summary_after_build) = self.summary_after_build {
            config.summary_after_build = summary_after_build;
        }
    }
}
