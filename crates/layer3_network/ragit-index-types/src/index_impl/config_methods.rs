use crate::index_struct::Index;
use ragit_types::{ApiError, map_serde_json_error};
use serde_json::Value;

impl Index {
    pub fn get_config_by_key(&self, key: String) -> Result<Value, ApiError> {
        let value = match key.as_str() {
            "model" => map_serde_json_error(serde_json::to_value(&self.api_config.model))?,
            "timeout" => map_serde_json_error(serde_json::to_value(&self.api_config.timeout))?,
            "sleep_between_retries" => {
                map_serde_json_error(serde_json::to_value(&self.api_config.sleep_between_retries))?
            }
            "max_retry" => map_serde_json_error(serde_json::to_value(&self.api_config.max_retry))?,
            "sleep_after_llm_call" => map_serde_json_error(serde_json::to_value(&self.api_config.sleep_after_llm_call))?,
            "dump_log" => map_serde_json_error(serde_json::to_value(&self.api_config.dump_log))?,
            "dump_api_usage" => map_serde_json_error(serde_json::to_value(&self.api_config.dump_api_usage))?,
            "enable_muse_mode" => map_serde_json_error(serde_json::to_value(&self.api_config.enable_muse_mode))?,
            "throttling_safety_margin" => {
                map_serde_json_error(serde_json::to_value(&self.api_config.throttling_safety_margin))?
            }
            "max_chunk_size" => map_serde_json_error(serde_json::to_value(&self.build_config.chunk_size))?,
            "max_summary_len" => map_serde_json_error(serde_json::to_value(&self.build_config.max_summary_len))?,
            "min_summary_len" => map_serde_json_error(serde_json::to_value(&self.build_config.min_summary_len))?,
            "enable_ii" => map_serde_json_error(serde_json::to_value(&self.query_config.enable_ii))?,
            _ => return Err(ApiError::InvalidConfigKey(key.clone())),
        };
        Ok(value)
    }
}