use crate::response::{IntoChatResponse, Response};

use ragit_types::ApiError as Error;
//use ragit_types::ApiError as Error;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct OpenAiResponse {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<OpenAiChoice>,
    usage: OpenAiUsage,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct OpenAiChoice {
    index: usize,
    message: OpenAiMessage,
    finish_reason: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct OpenAiUsage {
    prompt_tokens: usize,
    completion_tokens: usize,
    total_tokens: usize,

    // only on groq api
    #[serde(skip)]
    prompt_time: f32,
    #[serde(skip)]
    completion_time: f32,
    #[serde(skip)]
    total_time: f32,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct OpenAiMessage {
    role: String,
    content: String,
    reasoning_content: Option<String>,
}

impl IntoChatResponse for OpenAiResponse {
    fn into_chat_response(&self) -> Result<Response, Error> {
        Ok(Response {
            messages: self
                .choices
                .iter()
                .map(|choice| choice.message.content.to_string())
                .collect(),
            reasonings: self
                .choices
                .iter()
                .map(|choice| choice.message.reasoning_content.clone())
                .collect(),
            output_tokens: self.usage.completion_tokens,
            prompt_tokens: self.usage.prompt_tokens,
            total_tokens: self.usage.total_tokens,
        })
    }
}
