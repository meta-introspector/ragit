use serde::{Deserialize, Serialize};

use super::QualityExpectations;

/// There are 2 types for models: `Model` and `ModelRaw`. I know it's confusing, I'm sorry.
/// `Model` is the type ragit internally uses and `ModelRaw` is only for json serialization.
/// Long time ago, there was only `Model` type. But then I implemented `models.json` interface.
/// I wanted people to directly edit the json file and found that `Model` isn't intuitive to
/// edit directly. So I added this struct.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ModelRaw {
    /// Model name shown to user.
    /// `rag config --set model` also
    /// uses this name.
    pub name: String,

    /// Model name used for api requests.
    pub api_name: String,

    pub can_read_images: bool,

    /// `openai | cohere | anthropic | google`
    ///
    /// If you're using an openai-compatible
    /// api, set this to `openai`.
    pub api_provider: String,

    /// It's necessary if you're using an
    /// openai-compatible api. If it's not
    /// set, ragit uses the default url of
    /// each api provider.
    pub api_url: Option<String>,

    /// Dollars per 1 million input tokens.
    pub input_price: f64,

    /// Dollars per 1 million output tokens.
    pub output_price: f64,

    // FIXME: I set the default value to 180 seconds long ago.
    //        At that time, it's very common for LLMs to take
    //        1 ~ 2 minutes to respond. But now, nobody would
    //        wait 180 seconds. Do I have to reduce it?
    /// The number is in seconds.
    /// If not set, it's default to 180 seconds.
    #[serde(default)]
    pub api_timeout: Option<u64>,

    pub explanation: Option<String>,

    /// If you don't want to use an env var, you
    /// can hard-code your api key in this field.
    #[serde(default)]
    pub api_key: Option<String>,

    /// If you've hard-coded your api key,
    /// you don't have to set this. If neither
    /// `api_key`, nor `api_env_var` is set,
    /// it assumes that the model doesn't require
    /// an api key.
    pub api_env_var: Option<String>,

    // New fields for rate limits
    #[serde(default)]
    pub requests_per_minute: Option<u32>,
    #[serde(default)]
    pub requests_per_day: Option<u32>,
    #[serde(default)]
    pub tokens_per_minute: Option<u32>,
    #[serde(default)]
    pub tokens_per_day: Option<i32>, // Use i32 to allow -1 for no limit

    // New fields
    #[serde(default)]
    pub quality_expectations: Option<QualityExpectations>,
    #[serde(default)]
    pub expected_response_time_ms: Option<u64>,
    #[serde(default)]
    pub initial_score: Option<String>, // e.g., "unknown", "presupposed_moderate", "presupposed_high"

    // New fields for multiple API keys
    #[serde(default)]
    pub api_keys: Option<Vec<String>>,
    #[serde(default)]
    pub api_env_vars: Option<Vec<String>>,
    #[serde(skip)]
    pub current_key_index: usize,
    pub test_model: Option<super::model::test_model_struct::TestModel>,
}

impl ModelRaw {
    pub fn llama_70b() -> Self {
        // DEFAULT_MODELS.get("llama3.3-70b-groq").unwrap().clone()
        unimplemented!()
    }

    pub fn llama_8b() -> Self {
        // DEFAULT_MODELS.get("llama3.1-8b-groq").unwrap().clone()
        unimplemented!()
    }

    pub fn gpt_4o() -> Self {
        // DEFAULT_MODELS.get("gpt-4o").unwrap().clone()
        unimplemented!()
    }

    pub fn gpt_4o_mini() -> Self {
        // TODO: Replace with actual model loading from DEFAULT_MODELS
        Self {
            name: "gpt-4o-mini".to_string(),
            api_name: "gpt-4o-mini".to_string(),
            can_read_images: true,
            api_provider: "openai".to_string(),
            api_url: None,
            input_price: 0.0,
            output_price: 0.0,
            api_timeout: None,
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
            test_model: None,
        }
    }

    pub fn gemini_2_flash() -> Self {
        // TODO: Replace with actual model loading from DEFAULT_MODELS
        Self {
            name: "gemini-2.0-flash".to_string(),
            api_name: "gemini-2.0-flash".to_string(),
            can_read_images: true,
            api_provider: "google".to_string(),
            api_url: None,
            input_price: 0.0,
            output_price: 0.0,
            api_timeout: None,
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
            test_model: None,
        }
    }

    pub fn sonnet() -> Self {
        // DEFAULT_MODELS.get("claude-3.7-sonnet").unwrap().clone()
        unimplemented!()
    }

    pub fn phi_4_14b() -> Self {
        // DEFAULT_MODELS.get("phi-4-14b-ollama").unwrap().clone()
        unimplemented!()
    }

    pub fn command_r() -> Self {
        // DEFAULT_MODELS.get("command-r").unwrap().clone()
        unimplemented!()
    }

    pub fn command_r_plus() -> Self {
        // DEFAULT_MODELS.get("command-r-plus").unwrap().clone()
        unimplemented!()
    }

    pub fn default_models() -> Vec<ModelRaw> {
        // let mut models: Vec<ModelRaw> =
        //     DEFAULT_MODELS.values().map(|model| model.clone()).collect();
        // let _ = crate::rate_limit::merge_rate_limits(&mut models);
        // models
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    // use super::{DEFAULT_MODELS, ModelRaw};

    #[test]
    fn validate_models_dot_json() {
        // for model in DEFAULT_MODELS.values() {
        // Model::try_from(model).unwrap();
        // }
    }
}