use super::model_struct::Model;

impl PartialEq for Model {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.api_name == other.api_name
            && self.can_read_images == other.can_read_images
            && self.api_provider_name == other.api_provider_name
            && self.dollars_per_1b_input_tokens == other.dollars_per_1b_input_tokens
            && self.dollars_per_1b_output_tokens == other.dollars_per_1b_output_tokens
            && self.api_timeout == other.api_timeout
            && self.explanation == other.explanation
            && self.api_key == other.api_key
            && self.api_env_var == other.api_env_var
            && self.requests_per_minute == other.requests_per_minute
            && self.requests_per_day == other.requests_per_day
            && self.tokens_per_minute == other.tokens_per_minute
            && self.tokens_per_day == other.tokens_per_day
            && self.quality_expectations == other.quality_expectations
            && self.expected_response_time_ms == other.expected_response_time_ms
            && self.initial_score == other.initial_score
            && self.api_keys == other.api_keys
            && self.api_env_vars == other.api_env_vars
            && self.current_key_index == other.current_key_index
    }
}
