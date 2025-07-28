use super::model_struct::Model;

impl Model {
    /// This is a test model. It always returns a string `"dummy"`.
    pub fn dummy() -> Self {
        Model {
            name: String::from("dummy"),
            api_name: String::from("test-model-dummy-v0"),
            can_read_images: false,
            api_provider_name: String::from("test"),
            dollars_per_1b_input_tokens: 0,
            dollars_per_1b_output_tokens: 0,
            api_timeout: 180,
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
            test_model: Some(super::test_model_struct::TestModel { dummy_response: String::from("dummy") }),
        }
    }
}
