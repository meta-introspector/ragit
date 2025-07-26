use ragit_index_types::index_struct::Index;
use ragit_error::ApiError;
use serde_json::Value;
use std::collections::HashMap;
use ragit_types::api_config::ApiConfig;
use ragit_types::build_config::BuildConfig;
use ragit_types::query::QueryConfig;

impl Index {
    pub fn get_all_configs(&self) -> Result<Vec<(String, Value)>, ApiError> {
        let mut configs = vec![];

        // ApiConfig
        configs.push((
            "model".to_string(),
            serde_json::to_value(&self.api_config.model)?,
        ));
        configs.push((
            "timeout".to_string(),
            serde_json::to_value(&self.api_config.timeout)?,
        ));
        configs.push((
            "sleep_between_retries".to_string(),
            serde_json::to_value(&self.api_config.sleep_between_retries)?,
        ));
        configs.push((
            "max_retry".to_string(),
            serde_json::to_value(&self.api_config.max_retry)?,
        ));
        configs.push((
            "sleep_after_llm_call".to_string(),
            serde_json::to_value(&self.api_config.sleep_after_llm_call)?,
        ));
        configs.push((
            "dump_log".to_string(),
            serde_json::to_value(&self.api_config.dump_log)?,
        ));
        configs.push((
            "dump_api_usage".to_string(),
            serde_json::to_value(&self.api_config.dump_api_usage)?,
        ));
        configs.push((
            "enable_muse_mode".to_string(),
            serde_json::to_value(&self.api_config.enable_muse_mode)?,
        ));
        configs.push((
            "throttling_safety_margin".to_string(),
            serde_json::to_value(&self.api_config.throttling_safety_margin)?,
        ));

        // BuildConfig
        configs.push((
            "max_chunk_size".to_string(),
            serde_json::to_value(&self.build_config.chunk_size)?,
        ));
        configs.push((
            "max_summary_len".to_string(),
            serde_json::to_value(&self.build_config.max_summary_len)?,
        ));
        configs.push((
            "min_summary_len".to_string(),
            serde_json::to_value(&self.build_config.min_summary_len)?,
        ));

        // QueryConfig
        configs.push((
            "enable_ii".to_string(),
            serde_json::to_value(&self.query_config.enable_ii)?,
        ));

        Ok(configs)
    }
}