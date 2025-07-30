pub mod prelude;
pub mod index_struct;
pub mod index_impl;
pub mod load_mode;
pub use index_struct::Index;
pub use ragit_utils::constant;
pub use ragit_utils::constant::*;
pub use std::time::{Duration, Instant};
pub use ragit_types::chunk;
pub use ragit_types::uid::Uid;

pub mod word_counter;
pub fn index_get_prompt(
    index: &Index,
    prompt_name: &str,
) -> Result<String, ragit_error::ApiError> {
    match index.prompts.get(prompt_name) {
        Some(prompt) => Ok(prompt.to_string()),
        None => Err(ragit_error::ApiError::PromptMissing(prompt_name.to_string())),
    }
}

