use serde::{Deserialize, Serialize};
use crate::Uid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProcessedDoc {
    pub doc_id: Uid,
    pub tokens: Vec<String>,
}

impl ProcessedDoc {
    pub fn empty() -> Self {
        ProcessedDoc {
            doc_id: Uid::dummy(),
            tokens: Vec::new(),
        }
    }

    pub fn extend(&mut self, other: &ProcessedDoc) {
        self.tokens.extend(other.tokens.clone());
    }

    pub fn render(&self, term_only: bool, stat_only: bool, json_mode: bool) -> String {
        // Placeholder implementation
        format!("Placeholder for ProcessedDoc render: term_only={term_only}, stat_only={stat_only}, json_mode={json_mode}")
    }
}
