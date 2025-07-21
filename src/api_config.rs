use crate::constant::{INDEX_DIR_NAME, LOG_DIR_NAME};
use crate::error::Error;
use ragit_api::audit::{AuditRecord, AuditRecordAt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use crate::prelude::*;

// This struct is used for loading partial configurations from ~/.config/ragit/api.json
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PartialApiConfig {
    pub api_key: Option<String>,
    pub model: Option<String>,
    pub timeout: Option<u64>,
    pub sleep_between_retries: Option<u64>,
    pub max_retry: Option<usize>,
    pub sleep_after_llm_call: Option<u64>,
    pub dump_log: Option<bool>,
    pub dump_api_usage: Option<bool>,
    pub enable_muse_mode: Option<bool>,
    pub throttling_safety_margin: Option<f64>,
}

impl PartialEq for PartialApiConfig {
    fn eq(&self, other: &Self) -> bool {
        self.api_key == other.api_key &&
        self.model == other.model &&
        self.timeout == other.timeout &&
        self.sleep_between_retries == other.sleep_between_retries &&
        self.max_retry == other.max_retry &&
        self.sleep_after_llm_call == other.sleep_after_llm_call &&
        self.dump_log == other.dump_log &&
        self.dump_api_usage == other.dump_api_usage &&
        self.enable_muse_mode == other.enable_muse_mode &&
        self.throttling_safety_margin == other.throttling_safety_margin
    }
}

impl PartialApiConfig {
    // Apply partial config to a full config
    pub fn apply_to(&self, config: &mut ApiConfig) {
        if let Some(api_key) = &self.api_key {
            config.api_key = Some(api_key.clone());
        }
        if let Some(model) = &self.model {
            config.model = model.clone();
        }
        if let Some(timeout) = self.timeout {
            config.timeout = Some(timeout);
        }
        if let Some(sleep_between_retries) = self.sleep_between_retries {
            config.sleep_between_retries = sleep_between_retries.clone();
        }
        if let Some(max_retry) = self.max_retry {
            config.max_retry = max_retry;
        }
        if let Some(sleep_after_llm_call) = self.sleep_after_llm_call {
            config.sleep_after_llm_call = Some(sleep_after_llm_call);
        }
        if let Some(dump_log) = self.dump_log {
            config.dump_log = dump_log.clone();
        }
        if let Some(dump_api_usage) = self.dump_api_usage {
            config.dump_api_usage = dump_api_usage.clone();
        }
        if let Some(enable_muse_mode) = self.enable_muse_mode {
            config.enable_muse_mode = enable_muse_mode.clone();
        }
        if let Some(throttling_safety_margin) = self.throttling_safety_margin {
            config.throttling_safety_margin = throttling_safety_margin;
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiConfig {
    /// This value is NOT used anymore. It's here for backward-compatibility.
    /// You have to use env var or `models.json`.
    pub api_key: Option<String>,

    /// Run `rag ls-models` to see the list of the models.
    pub model: String,
    pub timeout: Option<u64>,
    pub sleep_between_retries: u64,
    pub max_retry: usize,

    /// It's in milliseconds.
    /// If you see 429 too often, use this option.
    pub sleep_after_llm_call: Option<u64>,

    /// It records every LLM conversation, including failed ones.
    /// It's useful if you wanna know what's going on!
    /// But be careful, it would take a lot of space.
    pub dump_log: bool,

    /// It records how many tokens are used.
    pub dump_api_usage: bool,

    /// It uses multiple prompts to get diverse results.
    pub enable_muse_mode: bool,
    pub throttling_safety_margin: f64,
}

impl PartialEq for ApiConfig {
    fn eq(&self, other: &Self) -> bool {
        self.api_key == other.api_key &&
        self.model == other.model &&
        self.timeout == other.timeout &&
        self.sleep_between_retries == other.sleep_between_retries &&
        self.max_retry == other.max_retry &&
        self.sleep_after_llm_call == other.sleep_after_llm_call &&
        self.dump_log == other.dump_log &&
        self.dump_api_usage == other.dump_api_usage &&
        self.enable_muse_mode == other.enable_muse_mode &&
        self.throttling_safety_margin == other.throttling_safety_margin
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        ApiConfig {
            api_key: None,
            dump_log: false,
            dump_api_usage: true,
            enable_muse_mode: false,
            max_retry: 5,
            sleep_between_retries: 15_000,
            timeout: Some(120_000),
            sleep_after_llm_call: None,
            model: String::from("llama3.3-70b-groq"),
            throttling_safety_margin: 0.9,
        }
    }
}

impl ApiConfig {
    pub fn create_pdl_path(&self, root_dir: &PathBuf, job: &str) -> Option<PathBuf> {
        let now = Local::now();

        self.dump_log_at(root_dir).as_ref().map(
            |path| path_utils::str_to_pathbuf(&join(
                &path_utils::pathbuf_to_str(path),
                &format!(
                    "{job}.pdl",
                ),
            ).unwrap())
        )
    }

    pub fn dump_log_at(&self, root_dir: &PathBuf) -> Option<PathBuf> {
        if self.dump_log {
            ragit_fs::join3(&ragit_utils::path_utils::pathbuf_to_str(root_dir), INDEX_DIR_NAME, LOG_DIR_NAME)
                .ok()
                .map(|s| ragit_utils::path_utils::str_to_pathbuf(&s))
        }

        else {
            None
        }
    }

    pub fn dump_api_usage_at(&self, root_dir: &PathBuf, id: &str) -> Option<AuditRecordAt> {
        if self.dump_api_usage {
            match ragit_fs::join3(&ragit_utils::path_utils::pathbuf_to_str(root_dir), INDEX_DIR_NAME, "usages.json") {
                Ok(path_str) => {
                    let path = ragit_utils::path_utils::str_to_pathbuf(&path_str);
                    if !ragit_fs::exists(&path) {
                        let _ = ragit_fs::write_string(
                            &ragit_utils::path_utils::pathbuf_to_str(&path),
                            "{}",
                            ragit_fs::WriteMode::AlwaysCreate,
                        );
                    }

                    Some(AuditRecordAt { path, id: id.to_string() })
                },
                Err(_) => None,
            }
        }

        else {
            None
        }
    }

    pub fn get_api_usage(&self, root_dir: &PathBuf, id: &str) -> Result<HashMap<String, AuditRecord>, Error> {
        match &self.dump_api_usage_at(root_dir, id) {
            Some(AuditRecordAt { path, id }) => {
                let tracker = ragit_api::audit::Tracker::load_from_file(&ragit_utils::path_utils::pathbuf_to_str(path))?;

                match tracker.0.get(id) {
                    Some(record) => Ok(record.clone()),

                    // It's not an error, it's just that this id was never used
                    None => Ok(HashMap::new()),
                }
            },
            None => Ok(HashMap::new()),  // TODO: is this an error attempting to do this? I'm not sure
        }
    }
}
