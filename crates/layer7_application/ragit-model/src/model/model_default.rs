use super::model_struct::Model;

impl Default for Model {
    fn default() -> Self {
        Model {
            name: String::new(),
            api_name: String::new(),
            can_read_images: false,
            api_provider_name: String::new(),
            dollars_per_1b_input_tokens: 0,
            dollars_per_1b_output_tokens: 0,
            api_timeout: 0,
            explanation: None,
            api_key: None,
            api_env_var: None,
            requests_per_minute: None,
            requests_per_day: None,
            tokens_per_minute: None,
            tokens_per_day: None,
            quality_expectations: None,
            expected_response_time_ms: None,
            initial_score: None,
            api_keys: None,
            api_env_vars: None,
            current_key_index: 0,
            api_url: None,
            test_model: None,
        }
    }
}
