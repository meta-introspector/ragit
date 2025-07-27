use super::model_struct::Model;
use crate::model_raw::ModelRaw;

impl From<&Model> for ModelRaw {
    fn from(m: &Model) -> ModelRaw {
        ModelRaw {
            name: m.name.clone(),
            api_name: m.api_name.clone(),
            can_read_images: m.can_read_images,
            api_provider: m.api_provider_name.clone(),

            // This field is for openai-compatible apis. The other api
            // providers do not need this field. The problem is that
            // `m.get_api_url()` may fail if api provider is google.
            // So it just ignores errors.
            api_url: m.get_api_url().ok(),

            input_price: m.dollars_per_1b_input_tokens as f64 / 1000.0,
            output_price: m.dollars_per_1b_output_tokens as f64 / 1000.0,
            api_timeout: Some(m.api_timeout),
            explanation: m.explanation.clone(),
            api_key: m.api_key.clone(),
            api_env_var: m.api_env_var.clone(),
            requests_per_minute: m.requests_per_minute,
            requests_per_day: m.requests_per_day,
            tokens_per_minute: m.tokens_per_minute,
            tokens_per_day: m.tokens_per_day,
            quality_expectations: m.quality_expectations.clone(),
            expected_response_time_ms: m.expected_response_time_ms,
            initial_score: m.initial_score.clone(),
            api_keys: m.api_keys.clone(),
            api_env_vars: m.api_env_vars.clone(),
            current_key_index: 0,
            test_model: m.test_model.clone(),
        }
    }
}
