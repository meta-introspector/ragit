use super::model_struct::Model;
use ragit_types::ApiError as Error;
use crate::model_raw::ModelRaw;

impl TryFrom<&ModelRaw> for Model {
    type Error = Error;

    fn try_from(m: &ModelRaw) -> Result<Model, Error> {
        Ok(Model {
            name: m.name.clone(),
            api_name: m.api_name.clone(),
            can_read_images: m.can_read_images,
            api_provider_name: m.api_provider.clone(),
            dollars_per_1b_input_tokens: (m.input_price * 1000.0).round() as u64,
            dollars_per_1b_output_tokens: (m.output_price * 1000.0).round() as u64,
            api_timeout: m.api_timeout.unwrap_or(180),
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
        })
    }
}
