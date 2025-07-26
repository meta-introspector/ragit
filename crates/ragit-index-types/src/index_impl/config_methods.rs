use crate::index_struct::Index;
use ragit_error::ApiError;
use serde_json::Value;
use ragit_types::api_config::ApiConfig;
use ragit_types::build_config::BuildConfig;
use ragit_types::query::QueryConfig;

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

    pub fn set_config_by_key(
        &mut self,
        key: String,
        value: String,
    ) -> Result<Option<Value>, ApiError> {
        let previous_value = match key.as_str() {
            "model" => {
                let prev = self.api_config.model.clone();
                self.api_config.model = value;
                serde_json::to_value(prev).ok()
            }
            "timeout" => {
                let prev = self.api_config.timeout;
                self.api_config.timeout = Some(value.parse()?);
                serde_json::to_value(prev).ok()
            }
            "sleep_between_retries" => {
                let prev = self.api_config.sleep_between_retries;
                self.api_config.sleep_between_retries = value.parse()?;
                serde_json::to_value(prev).ok()
            }
            "max_retry" => {
                let prev = self.api_config.max_retry;
                self.api_config.max_retry = value.parse()?;
                serde_json::to_value(prev).ok()
            }
            "sleep_after_llm_call" => {
                let prev = self.api_config.sleep_after_llm_call;
                self.api_config.sleep_after_llm_call = Some(value.parse()?);
                serde_json::to_value(prev).ok()
            }
            "dump_log" => {
                let prev = self.api_config.dump_log;
                self.api_config.dump_log = value.parse()?;
                serde_json::to_value(prev).ok()
            }
            "dump_api_usage" => {
                let prev = self.api_config.dump_api_usage;
                self.api_config.dump_api_usage = value.parse()?;
                serde_json::to_value(prev).ok()
            }
            "enable_muse_mode" => {
                let prev = self.api_config.enable_muse_mode;
                self.api_config.enable_muse_mode = value.parse()?;
                serde_json::to_value(prev).ok()
            }
            "throttling_safety_margin" => {
                let prev = self.api_config.throttling_safety_margin;
                self.api_config.throttling_safety_margin = value.parse()?;
                serde_json::to_value(prev).ok()
            }
            "max_chunk_size" => {
                let prev = self.build_config.chunk_size;
                self.build_config.chunk_size = value.parse()?;
                serde_json::to_value(prev).ok()
            }
            "max_summary_len" => {
                let prev = self.build_config.max_summary_len;
                self.build_config.max_summary_len = value.parse()?;
                serde_json::to_value(prev).ok()
            }
            "min_summary_len" => {
                let prev = self.build_config.min_summary_len;
                self.build_config.min_summary_len = value.parse()?;
                serde_json::to_value(prev).ok()
            }
            "enable_ii" => {
                let prev = self.query_config.enable_ii;
                self.query_config.enable_ii = value.parse()?;
                serde_json::to_value(prev).ok()
            }
            _ => return Err(ApiError::InvalidConfigKey(key.clone())),
        };
        // self.save_to_file(self.root_dir.join(INDEX_FILE_NAME))?;
        Ok(previous_value)
    }
}