// src/lib.rs

use serde::{Deserialize, Serialize};

// --- AugmentedTerm Struct (from idx/augmented_terms.jsonl) ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AugmentedTerm {
    pub term: String,
    pub count: u32,
    pub category: Option<String>,
    pub significance: Option<String>,
    pub vibe: Option<String>,
    pub action_suggestion: Option<String>,
    pub emoji_representation: Option<String>,
    pub semantic_names: Option<Vec<String>>,
    pub osi_layer: Option<String>,
    pub prime_factor: Option<u32>,
    pub is_power_of_two: Option<bool>,
    pub numerical_address: Option<u64>,
    pub embedding_vectors: Option<Vec<f32>>, // Assuming f32 for embeddings
    pub versions: Option<Vec<String>>,
    pub first_seen_timestamp: Option<i64>,
    pub last_seen_timestamp: Option<i64>,
}

// --- ParsingPhaseData Struct ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsingPhaseData {
    pub id: String,
    pub file_path: String,
    pub line: u32,
    pub column: u32,
    pub phase: String,
    pub processing_order: u32,
    pub element_type: String,
    pub element_name: Option<String>,
    pub element_signature: Option<String>,
    pub syntax_data: Option<String>,
    pub symbol_data: Option<String>,
    pub type_data: Option<String>,
    pub diagnostic_data: Option<String>,
    pub processing_time_ms: u64,
    pub timestamp: u64,
    pub rust_version: String,
    pub analyzer_version: String,
    pub source_snippet: String,
    pub context_before: Option<String>,
    pub context_after: Option<String>,
    // Associated AugmentedTerms
    #[serde(skip)] // Skip serialization for now, as it's derived
    pub augmented_terms: Vec<AugmentedTerm>,
}
