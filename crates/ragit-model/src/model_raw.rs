use serde::{Deserialize, Serialize};
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use super::QualityExpectations;

/// There are 2 types for models: `Model` and `ModelRaw`. I know it's confusing, I'm sorry.
/// `Model` is the type ragit internally uses and `ModelRaw` is only for json serialization.
/// Long time ago, there was only `Model` type. But then I implemented `models.json` interface.
/// I wanted people to directly edit the json file and found that `Model` isn't intuitive to
/// edit directly. So I added this struct.
#[derive(Debug, Deserialize, Serialize)]
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
    pub current_key_index: AtomicUsize,
}

impl Clone for ModelRaw {
    fn clone(&self) -> Self {
        ModelRaw {
            name: self.name.clone(),
            api_name: self.api_name.clone(),
            can_read_images: self.can_read_images,
            api_provider: self.api_provider.clone(),
            api_url: self.api_url.clone(),
            input_price: self.input_price,
            output_price: self.output_price,
            api_timeout: self.api_timeout,
            explanation: self.explanation.clone(),
            api_key: self.api_key.clone(),
            api_env_var: self.api_env_var.clone(),
            requests_per_minute: self.requests_per_minute,
            requests_per_day: self.requests_per_day,
            tokens_per_minute: self.tokens_per_minute,
            tokens_per_day: self.tokens_per_day,
            quality_expectations: self.quality_expectations.clone(),
            expected_response_time_ms: self.expected_response_time_ms,
            initial_score: self.initial_score.clone(),
            api_keys: self.api_keys.clone(),
            api_env_vars: self.api_env_vars.clone(),
            current_key_index: AtomicUsize::new(self.current_key_index.load(Ordering::SeqCst)),
        }
    }
}

impl PartialEq for ModelRaw {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.api_name == other.api_name
            && self.can_read_images == other.can_read_images
            && self.api_provider == other.api_provider
            && self.api_url == other.api_url
            && self.input_price == other.input_price
            && self.output_price == other.output_price
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
            && self.current_key_index.load(Ordering::SeqCst)
                == other.current_key_index.load(Ordering::SeqCst)
    }
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
        // DEFAULT_MODELS.get("gpt-4o-mini").unwrap().clone()
        unimplemented!()
    }

    pub fn gemini_2_flash() -> Self {
        // DEFAULT_MODELS.get("gemini-2.0-flash").unwrap().clone()
        unimplemented!()
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