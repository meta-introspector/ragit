use serde::{Deserialize, Serialize};

use crate::qa_system::quality_scores::QualityScores;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelQAResult {
    pub model_name: String,
    pub initial_score: String,
    pub api_key_used: String,
    pub response: String,
    pub response_time_ms: u64,
    pub tokens_used: u32,
    pub quality_scores: QualityScores,
    pub user_score: Option<f64>, // User-provided score after review
}
