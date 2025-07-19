use crate::api_provider::ApiProvider;
use crate::error::Error;
use ragit_fs::join4;

use super::{QualityExpectations, TestModel, ModelRaw};

#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    pub name: String,
    pub api_name: String,
    pub can_read_images: bool,
    pub api_provider: ApiProvider,
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
}

impl Model {
    /// This is a test model. It always returns a string `"dummy"`.
    pub fn dummy() -> Self {
        Model {
            name: String::from("dummy"),
            api_name: String::from("test-model-dummy-v0"),
            can_read_images: false,
            api_provider: ApiProvider::Test(TestModel::Dummy),
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
        }
    }

    /// This is a test model. It takes a response from you.
    pub fn stdin() -> Self {
        Model {
            name: String::from("stdin"),
            api_name: String::from("test-model-stdin-v0"),
            can_read_images: false,
            api_provider: ApiProvider::Test(TestModel::Stdin),
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
        }
    }

    /// This is a test model. It always throws an error.
    pub fn error() -> Self {
        Model {
            name: String::from("error"),
            api_name: String::from("test-model-error-v0"),
            can_read_images: false,
            api_provider: ApiProvider::Test(TestModel::Error),
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
        }
    }

    pub fn get_api_url(&self) -> Result<String, Error> {
        let url = match &self.api_provider {
            ApiProvider::Anthropic => String::from("https://api.anthropic.com/v1/messages"),
            ApiProvider::Cohere => String::from("https://api.cohere.com/v2/chat"),
            ApiProvider::OpenAi { url } => url.to_string(),
            ApiProvider::Google => format!(
                "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
                self.api_name,
                self.get_api_key()?,
            ),
            ApiProvider::Test(_) => String::new(),
        };

        Ok(url)
    }

    pub fn get_api_key(&self) -> Result<String, Error> {
        // First, check if the API key is directly set in the model
        if let Some(key) = &self.api_key {
            return Ok(key.to_string());
        }

        // Next, check if an environment variable is specified and try to get the API key from it
        if let Some(var) = &self.api_env_var {
            if let Ok(key) = std::env::var(var) {
                return Ok(key.to_string());
            }

            // Don't return an error yet, try the other methods first
        }

        // If we get here, try to find the API key in external model files
        if let Some(key) = self.find_api_key_in_external_files()? {
            return Ok(key);
        }

        // If we have an api_env_var but couldn't find the key anywhere, return an error
        if let Some(var) = &self.api_env_var {
            return Err(Error::ApiKeyNotFound { env_var: Some(var.to_string()) });
        }

        // If both `api_key` and `api_env_var` are not set,
        // it assumes that the model does not require an
        // api key.
        Ok(String::new())
    }

    fn find_api_key_in_external_files(&self) -> Result<Option<String>, Error> {
        // Try to find the API key in the file indicated by RAGIT_MODEL_FILE
        if let Ok(file_path) = std::env::var("RAGIT_MODEL_FILE") {
            if let Some(key) = self.find_api_key_in_file(&file_path)? {
                return Ok(Some(key));
            }
        }

        // Try to find the API key in ~/.config/ragit/models.json
        if let Ok(home_dir) = std::env::var("HOME") {
            let config_path = join4(
                &home_dir,
                ".config",
                "ragit",
                "models.json",
            )?;

            if let Some(key) = self.find_api_key_in_file(&config_path)? {
                return Ok(Some(key));
            }
        }

        Ok(None)
    }

    fn find_api_key_in_file(&self, file_path: &str) -> Result<Option<String>, Error> {
        use std::fs::File;
        use std::io::Read;

        // Check if the file exists
        let file = match File::open(file_path) {
            Ok(file) => file,
            Err(_) => return Ok(None), // File doesn't exist or can't be opened
        };

        // Read the file content
        let mut content = String::new();
        if let Err(_) = file.take(10_000_000).read_to_string(&mut content) {
            return Ok(None); // Can't read the file
        }

        // Parse the JSON
        let models: Vec<ModelRaw> = match serde_json::from_str(&content) {
            Ok(models) => models,
            Err(_) => return Ok(None), // Can't parse the JSON
        };

        // Find the model with the same name
        for model in models {
            if model.name == self.name {
                // If the model has an API key, return it
                if let Some(key) = model.api_key {
                    return Ok(Some(key));
                }

                // If the model has an environment variable, try to get the API key from it
                if let Some(var) = model.api_env_var {
                    if let Ok(key) = std::env::var(&var) {
                        return Ok(Some(key));
                    }
                }
            }
        }

        Ok(None)
    }

    pub fn is_test_model(&self) -> bool {
        matches!(self.api_provider, ApiProvider::Test(_))
    }

    pub fn default_models() -> Vec<Model> {
        ModelRaw::default_models().iter().map(
            |model| model.try_into().unwrap()
        ).collect()
    }
}

impl TryFrom<&ModelRaw> for Model {
    type Error = Error;

    fn try_from(m: &ModelRaw) -> Result<Model, Error> {
        Ok(Model {
            name: m.name.clone(),
            api_name: m.api_name.clone(),
            can_read_images: m.can_read_images,
            api_provider: ApiProvider::parse(
                &m.api_provider,
                &m.api_url,
            )?,
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
        })
    }
}

impl From<&Model> for ModelRaw {
    fn from(m: &Model) -> ModelRaw {
        ModelRaw {
            name: m.name.clone(),
            api_name: m.api_name.clone(),
            can_read_images: m.can_read_images,
            api_provider: m.api_provider.to_string(),

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
        }
    }
}
