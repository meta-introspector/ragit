use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub enum SearchType {
    Exact,
    Tfidf,
}
