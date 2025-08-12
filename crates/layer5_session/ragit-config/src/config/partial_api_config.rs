use serde::{Deserialize, Serialize};
use ragit_types::api_config::ApiConfig;

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
