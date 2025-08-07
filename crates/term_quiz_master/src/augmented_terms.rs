use serde::{Serialize, Deserialize};
use super::augmented_term_entry::AugmentedTermEntry;

#[derive(Debug, Serialize, Deserialize)]
pub struct AugmentedTerms {
    pub augmented_terms: Vec<AugmentedTermEntry>,
}
