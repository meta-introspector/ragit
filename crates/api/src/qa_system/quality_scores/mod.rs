use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct QualityScores {
    pub accuracy: f64,
    pub coherence: f64,
    pub relevance: f64,
}
