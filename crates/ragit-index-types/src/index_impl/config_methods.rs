use crate::index_struct::Index;
use ragit_types::ApiError;
use serde_json::Value;

impl Index {
    pub fn get_config_by_key(&self, key: String) -> Result<Value, ApiError> {
        let value = match key.as_str() {
            "model" => serde_json::to_value(&self.api_config.model)?,
            "timeout" => serde_json::to_value(&self.api_config.timeout)?,
            "sleep_between_retries" => {
                serde_json::to_value(&self.api_config.sleep_between_retries)?
            }
            "max_retry" => serde_json::to_value(&self.api_config.max_retry)?,
            "sleep_after_llm_call" => serde_json::to_value(&self.api_config.sleep_after_llm_call)?,
            "dump_log" => serde_json::to_value(&self.api_config.dump_log)?,
            "dump_api_usage" => serde_json::to_value(&self.api_config.dump_api_usage)?,
            "enable_muse_mode" => serde_json::to_value(&self.api_config.enable_muse_mode)?,
            "throttling_safety_margin" => {
                serde_json::to_value(&self.api_config.throttling_safety_margin)?
            }
            "max_chunk_size" => serde_json::to_value(&self.build_config.chunk_size)?,
            "max_summary_len" => serde_json::to_value(&self.build_config.max_summary_len)?,
            "min_summary_len" => serde_json::to_value(&self.build_config.min_summary_len)?,
            "enable_ii" => serde_json::to_value(&self.query_config.enable_ii)?,
            _ => return Err(ApiError::InvalidConfigKey(key.clone())),
        };
        Ok(value)
    }
}