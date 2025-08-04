

use ragit_macros::OurMacro;

#[derive(Debug, Clone, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// A request to an LLM to generate embeddings for a series of tokens at specific layers.
pub struct EmbeddingRequest {
    /// The tokens to be processed by the LLM.
    pub tokens: Vec<String>,
    /// The depths (layer numbers) from which to sample the embeddings.
    pub layer_depths: Vec<u32>,
}
