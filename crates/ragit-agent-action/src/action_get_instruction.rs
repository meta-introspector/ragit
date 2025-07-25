use super::action_enum::Action;
use crate::constants;
use ragit_index_types::index_struct::Index;
use ragit_types::ApiError;
use anyhow::Result;

impl Action {
    pub(crate) fn get_instruction(&self, index: &Index) -> Result<String, ApiError> {
        let s = match self {
            Action::ReadFile => String::from(constants::INSTRUCTION_READ_FILE),
            Action::ReadDir => String::from(constants::INSTRUCTION_READ_DIR),
            Action::ReadChunk => String::from(constants::INSTRUCTION_READ_CHUNK),
            Action::SearchExact => String::from(constants::INSTRUCTION_SEARCH_EXACT),
            Action::SearchTfidf => String::from(constants::INSTRUCTION_SEARCH_TFIDF),
            Action::GetMeta => format!("{}", constants::INSTRUCTION_GET_META),
            Action::GetSummary => String::from(constants::INSTRUCTION_GET_SUMMARY),
            Action::SimpleRag => String::from(constants::INSTRUCTION_SIMPLE_RAG),
        };

        Ok(s)
    }
}
