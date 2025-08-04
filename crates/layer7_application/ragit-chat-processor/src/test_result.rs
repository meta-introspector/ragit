use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TestResult {
    pub success: bool,
    pub error_message: Option<String>,
    pub execution_time_ms: u64,
}
