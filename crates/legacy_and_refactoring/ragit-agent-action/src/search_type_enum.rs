use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, PartialEq, Deserialize)]
pub enum SearchType {
    Exact,
    Tfidf,
}
