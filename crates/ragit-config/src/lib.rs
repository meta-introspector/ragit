use serde::{Deserialize, Serialize};
use ragit_utils::error::Error;
use ragit_utils::ragit_path_utils::join_paths;
use ragit_fs::{exists, read_string};
use std::path::PathBuf;
use ragit_types::api_config::{ApiConfig};

// This struct is used for loading partial configurations from ~/.config/ragit/build.json
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
    // Apply partial config to a full config
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

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BuildConfig {
    /// It's not a max_chunk_size, and it's impossible to make every chunk have the same size because
    ///
    /// 1. An image cannot be splitted.
    /// 2. Different files cannot be merged.
    ///
    /// But it's guaranteed that a chunk is never bigger than chunk_size * 2.
    pub chunk_size: usize,

    pub slide_len: usize,

    /// An image is treated like an N characters string, and this is N.
    pub image_size: usize,

    /// It forces the LLM to generate a summary that has at least `min_summary_len` characters
    /// and at most `max_summary_len` characters.
    pub min_summary_len: usize,
    pub max_summary_len: usize,

    /// If it's set, `rag build` panics if there's any error with a file.
    /// For example, if there's an invalid utf-8 character `PlainTextReader` would die.
    /// If it cannot follow a link of an image in a markdown file, it would die.
    /// You don't need this option unless you're debugging ragit itself.
    pub strict_file_reader: bool,

    /// If the `.chunks` file is bigger than this (in bytes), the file is compressed
    pub compression_threshold: u64,

    /// 0 ~ 9
    pub compression_level: u32,

    /// If it's set, it runs `rag summary` after `rag build` is complete.
    #[serde(default = "_true")]
    pub summary_after_build: bool,
}

fn _true() -> bool {
    true
}

impl Default for BuildConfig {
    fn default() -> Self {
        BuildConfig {
            chunk_size: 4_000,
            slide_len: 1_000,
            image_size: 2_000,
            min_summary_len: 200,
            max_summary_len: 1000,
            strict_file_reader: false,
            compression_threshold: 2048,
            compression_level: 3,
            summary_after_build: true,
        }
    }
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PartialApiConfig {
    pub model: Option<String>,
    pub max_retry: Option<usize>,
    pub sleep_between_retries: Option<u64>,
    pub timeout: Option<u64>,
    pub sleep_after_llm_call: Option<u64>,
    pub dump_log: Option<bool>,
    pub dump_api_usage: Option<bool>,
    pub enable_muse_mode: Option<bool>,
    pub throttling_safety_margin: Option<f32>,
}

impl PartialApiConfig {
    pub fn apply_to(&self, config: &mut ApiConfig) {
        if let Some(model) = &self.model {
            config.model = model.clone();
        }
        if let Some(max_retry) = self.max_retry {
            config.max_retry = max_retry;
        }
        if let Some(sleep_between_retries) = self.sleep_between_retries {
            config.sleep_between_retries = sleep_between_retries;
        }
        if let Some(timeout) = self.timeout {
            config.timeout = Some(timeout);
        }
        if let Some(sleep_after_llm_call) = self.sleep_after_llm_call {
            config.sleep_after_llm_call = Some(sleep_after_llm_call);
        }
        if let Some(dump_log) = self.dump_log {
            config.dump_log = dump_log;
        }
        if let Some(dump_api_usage) = self.dump_api_usage {
            config.dump_api_usage = dump_api_usage;
        }
        if let Some(enable_muse_mode) = self.enable_muse_mode {
            config.enable_muse_mode = enable_muse_mode;
        }
        if let Some(throttling_safety_margin) = self.throttling_safety_margin {
            config.throttling_safety_margin = throttling_safety_margin;
        }
    }
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PartialQueryConfig {
    pub enable_ii: Option<bool>,
}

impl PartialQueryConfig {
    pub fn apply_to(&self, config: &mut ragit_utils::query::QueryConfig) {
        if let Some(enable_ii) = self.enable_ii {
            config.enable_ii = enable_ii;
        }
    }
}

/// Attempts to load a config file from ~/.config/ragit/
pub fn load_config_from_home<T: serde::de::DeserializeOwned>(
    filename: &str,
) -> Result<Option<T>, Error> {
    // Check for HOME environment variable
    let home_dir = match std::env::var("HOME") {
        Ok(path) => path,
        Err(_) => {
            eprintln!(
                "Warning: HOME environment variable not set, cannot check ~/.config/ragit/{}",
                filename
            );
            return Ok(None);
        }
    };

    let config_path = join_paths(
        &PathBuf::from(home_dir),
        &join_paths(
            &PathBuf::from(".config"),
            &join_paths(&PathBuf::from("ragit"), &PathBuf::from(filename))?,
        )?,
    )?;

    if exists(&config_path.clone().into()) {
        // Load from ~/.config/ragit/filename
        let config_content = read_string(config_path.to_str().unwrap())?;
        match serde_json::from_str::<T>(&config_content) {
            Ok(config) => {
                eprintln!(
                    "Info: Using configuration from ~/.config/ragit/{}",
                    filename
                );
                return Ok(Some(config));
            }
            Err(e) => {
                eprintln!(
                    "Warning: Could not parse {} from ~/.config/ragit/{}: {}",
                    filename, filename, e
                );
            }
        }
    }

    Ok(None)
}

pub fn load_api_config_from_home() -> Result<Option<PartialApiConfig>, Error> {
    load_config_from_home("api.json")
}

pub fn load_query_config_from_home() -> Result<Option<PartialQueryConfig>, Error> {
    load_config_from_home("query.json")
}

pub fn load_build_config_from_home() -> Result<Option<PartialBuildConfig>, Error> {
    load_config_from_home("build.json")
}
