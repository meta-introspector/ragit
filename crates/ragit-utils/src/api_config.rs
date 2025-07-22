use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ragit_types::AuditRecordAt;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ApiConfig {
    pub model: String,
    pub max_retry: usize,
    pub sleep_between_retries: u64,
    pub timeout: Option<u64>,
    pub sleep_after_llm_call: Option<u64>,
    pub dump_log: bool,
    pub dump_api_usage: bool,
    pub enable_muse_mode: bool,
    pub throttling_safety_margin: f32,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            model: "default_model".to_string(),
            max_retry: 5,
            sleep_between_retries: 1000,
            timeout: None,
            sleep_after_llm_call: None,
            dump_log: false,
            dump_api_usage: false,
            enable_muse_mode: false,
            throttling_safety_margin: 0.1,
        }
    }
}

impl ApiConfig {
    pub fn dump_api_usage_at(&self, root_dir: &PathBuf, name: &str) -> Option<AuditRecordAt> {
        if self.dump_api_usage {
            Some(AuditRecordAt {
                path: self.get_audit_path(root_dir.to_str().unwrap()),
                id: name.to_string(),
            })
        } else {
            None
        }
    }

    pub fn create_pdl_path(&self, _root_dir: &PathBuf, _name: &str) -> Option<PathBuf> {
        None
    }

    pub fn dump_log_at(&self, _root_dir: &PathBuf) -> Option<PathBuf> {
        None
    }

    fn get_audit_path(&self, root_dir: &str) -> String {
        // This should be a path to a file where audit logs are stored.
        // For now, let's use a placeholder.
        // TODO: Make this configurable or derive from constants.
        format!("{}/.ragit/audit.json", root_dir)
    }
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PartialApiConfig;

impl PartialApiConfig {
    pub fn apply_to(&self, _config: &mut ApiConfig) {
        // TODO: Implement actual application logic
    }
}