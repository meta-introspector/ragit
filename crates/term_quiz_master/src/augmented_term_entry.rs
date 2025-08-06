use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AugmentedTermEntry {
    pub term: String,
    pub count: usize,
    pub category: String,
    pub significance: String,
    pub vibe: String,
    pub action_suggestion: String,
    pub emoji_representation: Option<String>,
    pub semantic_names: Option<Vec<String>>,
    pub osi_layer: Option<String>,
    pub prime_factor: Option<usize>,
    pub is_power_of_two: Option<bool>,
    pub numerical_address: Option<usize>,
    pub embedding_vectors: Option<std::collections::HashMap<String, Vec<f64>>>,
    pub versions: Vec<TermVersionInfo>,
    pub first_seen_timestamp: Option<i64>,
    pub last_seen_timestamp: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TermVersionInfo {
    pub commit_hash: String,
    pub commit_timestamp: i64,
    pub weight: f64,
}
