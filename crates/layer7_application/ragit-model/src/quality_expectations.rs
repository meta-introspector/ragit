use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct QualityExpectations {
    pub accuracy: f64,
    pub coherence: f64,
    pub relevance: f64,
}
