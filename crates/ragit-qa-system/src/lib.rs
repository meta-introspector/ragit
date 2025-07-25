use serde::{Deserialize, Serialize};

pub mod prelude;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ModelQAResult {
    // Placeholder
    pub score: f32,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ModelQASystem {
    // Placeholder
    pub models: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct QualityScores {
    // Placeholder
    pub score: f32,
}
