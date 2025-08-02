use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use ragit_macros::OurMacro;

#[derive(Debug, Clone, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// The response from an LLM containing the sampled embeddings.
pub struct EmbeddingResponse {
    /// The embeddings, stored as a map from layer depth to a vector of embedding vectors.
    pub embeddings: HashMap<u32, Vec<Vec<f32>>>,
}
