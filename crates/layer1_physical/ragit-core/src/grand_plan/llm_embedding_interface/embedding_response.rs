/// The response from an LLM containing the sampled embeddings.
#[derive(Debug, Clone)]
pub struct EmbeddingResponse {
    /// The embeddings, stored as a map from layer depth to a vector of embedding vectors.
    pub embeddings: std::collections::HashMap<u32, Vec<Vec<f32>>>,
}
