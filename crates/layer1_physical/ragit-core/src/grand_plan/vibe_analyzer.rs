use crate::grand_plan::llm_sampling_system::embedding_sampler::EmbeddingSampler;
use crate::grand_plan::llm_embedding_interface::{EmbeddingRequest, EmbeddingResponse};

/// Analyzes the "vibe" of a sequence of emojis by getting their LLM embeddings.
pub struct VibeAnalyzer {
    embedding_sampler: EmbeddingSampler,
}

impl VibeAnalyzer {
    pub fn new(embedding_sampler: EmbeddingSampler) -> Self {
        VibeAnalyzer { embedding_sampler }
    }

    /// Gets the "vibe" (embeddings) of a sequence of emojis.
    pub fn get_emoji_vibe(&self, emojis: &[char], layer_depths: &[u32]) -> EmbeddingResponse {
        let tokens: Vec<String> = emojis.iter().map(|&c| c.to_string()).collect();
        let request = EmbeddingRequest {
            tokens,
            layer_depths: layer_depths.to_vec(),
        };
        self.embedding_sampler.sample_embeddings(&request)
    }
}
