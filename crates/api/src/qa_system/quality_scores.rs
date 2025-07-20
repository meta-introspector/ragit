use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QualityScores {
    pub accuracy: f64,
    pub coherence: f64,
    pub relevance: f64,
}
