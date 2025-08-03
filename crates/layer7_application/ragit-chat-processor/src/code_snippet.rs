use serde::{Serialize, Deserialize};
use crate::TestResult;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CodeSnippet {
    pub content: String,
    pub content_hash: String,
    pub language: String,
    pub token_count: usize,
    pub line_count: usize,
    pub char_count: usize,
    pub test_result: Option<TestResult>,
}
