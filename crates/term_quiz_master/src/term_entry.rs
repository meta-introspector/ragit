use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TermEntry {
    pub term: String,
    pub count: usize,
}
