use super::action_enum::Action;
use super::search_type_enum::SearchType;

impl From<Action> for SearchType {
    fn from(a: Action) -> SearchType {
        match a {
            Action::SearchExact => SearchType::Exact,
            Action::SearchTfidf => SearchType::Tfidf,
            _ => panic!(),
        }
    }
}
