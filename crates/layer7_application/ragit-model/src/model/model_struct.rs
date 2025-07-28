use serde::{Deserialize, Serialize};

use crate::prelude::{TestModel, *};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Model {
    pub api_url: Option<String>,
    pub test_model: Option<TestModel>,
    pub name: String,
    pub api_name: String,
    pub can_read_images: bool,
    pub api_provider_name: String,
    pub dollars_per_1b_input_tokens: u64,
    pub dollars_per_1b_output_tokens: u64,
    pub api_timeout: u64,
    pub explanation: Option<String>,
    pub api_key: Option<String>,
    pub api_env_var: Option<String>,
    pub requests_per_minute: Option<u32>,
    pub requests_per_day: Option<u32>,
    pub tokens_per_minute: Option<u32>,
    pub tokens_per_day: Option<i32>,
    pub quality_expectations: Option<QualityExpectations>,
    pub expected_response_time_ms: Option<u64>,
    pub initial_score: Option<String>,
    pub api_keys: Option<Vec<String>>,
    pub api_env_vars: Option<Vec<String>>,
    pub current_key_index: usize,
}
