use serde::{Serialize, Deserialize};
use super::term_entry::TermEntry;

#[derive(Debug, Serialize, Deserialize)]
pub struct TermReport {
    pub terms: Vec<TermEntry>,
}
