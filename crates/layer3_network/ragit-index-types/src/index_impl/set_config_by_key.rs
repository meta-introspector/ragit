use crate::index_struct::Index;
use ragit_types::ApiError;
use serde_json::Value;

pub fn index_set_config_by_key(
    index: &mut Index,
    key: String,
    value: String,
) -> Result<Option<Value>, ApiError> {
    let previous_value = match key.as_str() {
        "model" => {
            let prev = index.api_config.model.clone();
            index.api_config.model = value;
            serde_json::to_value(prev).ok()
        }
        "timeout" => {
            let prev = index.api_config.timeout;
            index.api_config.timeout = Some(value.parse()?);
            serde_json::to_value(prev).ok()
        }
        "sleep_between_retries" => {
            let prev = index.api_config.sleep_between_retries;
            index.api_config.sleep_between_retries = value.parse()?;
            serde_json::to_value(prev).ok()
        }
        "max_retry" => {
            let prev = index.api_config.max_retry;
            index.api_config.max_retry = value.parse()?;
            serde_json::to_value(prev).ok()
        }
        "sleep_after_llm_call" => {
            let prev = index.api_config.sleep_after_llm_call;
            index.api_config.sleep_after_llm_call = Some(value.parse()?);
            serde_json::to_value(prev).ok()
        }
        "dump_log" => {
            let prev = index.api_config.dump_log;
            index.api_config.dump_log = value.parse()?;
            serde_json::to_value(prev).ok()
        }
        "dump_api_usage" => {
            let prev = index.api_config.dump_api_usage;
            index.api_config.dump_api_usage = value.parse()?;
            serde_json::to_value(prev).ok()
        }
        "enable_muse_mode" => {
            let prev = index.api_config.enable_muse_mode;
            index.api_config.enable_muse_mode = value.parse()?;
            serde_json::to_value(prev).ok()
        }
        "throttling_safety_margin" => {
            let prev = index.api_config.throttling_safety_margin;
            index.api_config.throttling_safety_margin = value.parse()?;
            serde_json::to_value(prev).ok()
        }
        "max_chunk_size" => {
            let prev = index.build_config.chunk_size;
            index.build_config.chunk_size = value.parse()?;
            serde_json::to_value(prev).ok()
        }
        "max_summary_len" => {
            let prev = index.build_config.max_summary_len;
            index.build_config.max_summary_len = value.parse()?;
            serde_json::to_value(prev).ok()
        }
        "min_summary_len" => {
            let prev = index.build_config.min_summary_len;
            index.build_config.min_summary_len = value.parse()?;
            serde_json::to_value(prev).ok()
        }
        "enable_ii" => {
            let prev = index.query_config.enable_ii;
            index.query_config.enable_ii = value.parse()?;
            serde_json::to_value(prev).ok()
        }
        _ => return Err(ApiError::InvalidConfigKey(key.clone())),
    };
    // index.save_to_file(index.root_dir.join(INDEX_FILE_NAME))?;
    Ok(previous_value)
}