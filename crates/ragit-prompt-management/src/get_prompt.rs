use std::collections::HashMap;
use std::path::PathBuf;
use ragit_types::ApiError;

pub fn get_prompt(prompts: &HashMap<String, String>, _root_dir: &PathBuf, prompt_name: &str) -> Result<String, ApiError> {
    match prompts.get(prompt_name) {
        Some(prompt) => Ok(prompt.to_string()),
        None => Err(ApiError::PromptMissing(prompt_name.to_string())),
    }
}
