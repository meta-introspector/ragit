use super::action_enum::Action;

impl Action {
    pub fn all_actions() -> Vec<Action> {
        vec![
            Action::ReadFile,
            Action::ReadDir,
            Action::ReadChunk,
            Action::SearchExact,
            Action::SearchTfidf,
            Action::GetMeta,
            Action::GetSummary,
            Action::SimpleRag,
        ]
    }
}
