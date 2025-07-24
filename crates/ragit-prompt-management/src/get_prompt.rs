use crate::prelude::*;
use ragit_index_types::Index;

pub fn get_prompt(index: &Index, prompt_name: &str) -> Result<String, ApiError> {
    match index.prompts.get(prompt_name) {
        Some(prompt) => Ok(prompt.to_string()),
        None => Err(ApiError::PromptMissing(prompt_name.to_string())),
    }
}
