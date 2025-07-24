use crate::prelude::*;

pub trait IntoChatResponse {
    fn into_chat_response(&self) -> Result<Response, Error>;
}

mod anthropic;
mod cohere;
mod google;
mod openai;

pub use anthropic::AnthropicResponse;
pub use cohere::CohereResponse;
pub use google::GoogleResponse;
pub use openai::OpenAiResponse;

pub struct Response {
    messages: Vec<String>,
    reasonings: Vec<Option<String>>,
    output_tokens: usize,
    prompt_tokens: usize,
    total_tokens: usize,
}

impl Response {
    pub fn dummy(s: String) -> Self {
        Response {
            messages: vec![s],
            reasonings: vec![None],
            output_tokens: 0,
            prompt_tokens: 0,
            total_tokens: 0,
        }
    }

    pub fn from_str(s: &str, api_provider: &ApiProvider) -> Result<Self, Error> {
        let json: serde_json::Value = api_provider.parse_chat_response(s)?;
        match api_provider {
            ApiProvider::OpenAi { .. } => {
                let response: OpenAiResponse = serde_json::from_value(json)?;
                response.into_chat_response()
            }
            ApiProvider::Google => {
                let response: GoogleResponse = serde_json::from_value(json)?;
                response.into_chat_response()
            }
            ApiProvider::Cohere => {
                let response: CohereResponse = serde_json::from_value(json)?;
                response.into_chat_response()
            }
            ApiProvider::Anthropic => {
                let response: AnthropicResponse = serde_json::from_value(json)?;
                response.into_chat_response()
            }
            ApiProvider::Test(test_model) => {
                let response_str = test_model.get_dummy_response(&vec![])?;
                Ok(Response::dummy(response_str))
            }
        }
    }

    pub fn get_output_token_count(&self) -> usize {
        self.output_tokens
    }

    pub fn get_prompt_token_count(&self) -> usize {
        self.prompt_tokens
    }

    pub fn get_total_token_count(&self) -> usize {
        self.total_tokens
    }

    pub fn get_message(&self, index: usize) -> Option<&str> {
        self.messages.get(index).map(|s| s.as_str())
    }

    pub fn get_reasoning(&self, index: usize) -> Option<&str> {
        match self.reasonings.get(index) {
            Some(s) => s.as_ref().map(|s| s.as_str()),
            None => None,
        }
    }
}
