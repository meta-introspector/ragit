use super::action_enum::Action;
use crate::constants;

impl Action {
    pub(crate) fn write_unit_prompt(&self) -> String {
        match self {
            Action::ReadFile => constants::PROMPT_READ_FILE,
            Action::ReadDir => constants::PROMPT_READ_DIR,
            Action::ReadChunk => constants::PROMPT_READ_CHUNK,
            Action::SearchExact => constants::PROMPT_SEARCH_EXACT,
            Action::SearchTfidf => constants::PROMPT_SEARCH_TFIDF,
            Action::GetMeta => constants::PROMPT_GET_META,
            Action::GetSummary => constants::PROMPT_GET_SUMMARY,
            Action::SimpleRag => constants::PROMPT_SIMPLE_RAG,
        }.to_string()
    }
}
