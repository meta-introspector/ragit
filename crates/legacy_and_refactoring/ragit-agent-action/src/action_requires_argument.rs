use super::action_enum::Action;

impl Action {
    pub(crate) fn requires_argument(&self) -> bool {
        match self {
            Action::ReadFile => true,
            Action::ReadDir => true,
            Action::ReadChunk => true,
            Action::SearchExact => true,
            Action::SearchTfidf => true,
            Action::GetMeta => true,
            Action::GetSummary => false,
            Action::SimpleRag => true,
        }
    }
}
