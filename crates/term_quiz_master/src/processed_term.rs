// crates/term_quiz_master/src/processed_term.rs

#[derive(Debug, Clone)]
pub struct ProcessedTerm {
    pub term: String,
    pub count: usize,
    pub first_occurrence_index: usize,
    pub commit_short_id: String,
    pub commit_timestamp: i64,
    pub branch_name: String,
    pub submodule_path: Option<String>,
    pub thread_id: String,
    pub is_globally_introduced: bool,
}
