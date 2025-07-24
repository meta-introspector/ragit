use crate::prelude::*;

impl Index {
    pub fn get_prompt(&self, prompt_name: &str) -> Result<String, ApiError> {
        match self.prompts.get(prompt_name) {
            Some(prompt) => Ok(prompt.to_string()),
            None => Err(ApiError::PromptMissing(prompt_name.to_string())),
        }
    }
}
