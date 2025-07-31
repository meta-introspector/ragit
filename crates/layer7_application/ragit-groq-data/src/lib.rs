// use lazy_static::lazy_static;
// use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

// #[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
// pub struct ModelRaw {
//     pub name: String,
//     pub api_name: String,
//     pub can_read_images: bool,
//     pub api_provider: String,
//     pub api_url: Option<String>,
//     pub input_price: f64,
//     pub output_price: f64,
//     pub api_timeout: Option<u64>,
//     pub explanation: Option<String>,
//     pub api_key: Option<String>,
//     pub api_env_var: Option<String>,
//     pub requests_per_minute: Option<u32>,
//     pub requests_per_day: Option<u32>,
//     pub tokens_per_minute: Option<u32>,
//     pub tokens_per_day: Option<i32>,
//     pub quality_expectations: Option<super::QualityExpectations>,
//     pub expected_response_time_ms: Option<u64>,
//     pub initial_score: Option<String>,
//     pub api_keys: Option<Vec<String>>,
//     pub api_env_vars: Option<Vec<String>>,
//     #[serde(skip)]
//     pub current_key_index: std::sync::atomic::AtomicUsize,
// }

// lazy_static! {
//     pub static ref DEFAULT_MODELS: HashMap<String, ModelRaw> = {
//         let models_dot_json = include_str!("../../models.json");
//         let models = serde_json::from_str::<Vec<ModelRaw>>(&models_dot_json).unwrap();
//         models
//             .into_iter()
//             .map(|model| (model.name.clone(), model))
//             .collect()
//     };
// }

// impl ModelRaw {
//     pub fn llama_70b() -> Self {
//         DEFAULT_MODELS.get("llama3.3-70b-groq").unwrap().clone()
//     }

//     pub fn llama_8b() -> Self {
//         DEFAULT_MODELS.get("llama3.1-8b-groq").unwrap().clone()
//     }

//     pub fn gpt_4o() -> Self {
//         DEFAULT_MODELS.get("gpt-4o").unwrap().clone()
//     }

//     pub fn gpt_4o_mini() -> Self {
//         DEFAULT_MODELS.get("gpt-4o-mini").unwrap().clone()
//     }

//     pub fn gemini_2_flash() -> Self {
//         DEFAULT_MODELS.get("gemini-2.0-flash").unwrap().clone()
//     }

//     pub fn sonnet() -> Self {
//         DEFAULT_MODELS.get("claude-3.7-sonnet").unwrap().clone()
//     }

//     pub fn phi_4_14b() -> Self {
//         DEFAULT_MODELS.get("phi-4-14b-ollama").unwrap().clone()
//     }

//     pub fn command_r() -> Self {
//         DEFAULT_MODELS.get("command-r").unwrap().clone()
//     }

//     pub fn command_r_plus() -> Self {
//         DEFAULT_MODELS.get("command-r-plus").unwrap().clone()
//     }

//     pub fn default_models() -> Vec<ModelRaw> {
//         let models: Vec<ModelRaw> =
//             DEFAULT_MODELS.values().map(|model| model.clone()).collect();
//         // let _ = crate::rate_limit::merge_rate_limits(&mut models);
//         models
//     }
// }

pub fn dummy_groq_data() -> Result<(), anyhow::Error> {
    panic!("FIX ME LATER: Fix the bootstrap first and this code later.");
}