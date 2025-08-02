use crate::grand_plan::introspection_system::introspection_stream::IntrospectionStream;
use crate::grand_plan::vibe_analyzer::VibeAnalyzer;
use crate::grand_plan::llm_sampling_system::llm_model::LlmModel;
use crate::grand_plan::llm_sampling_system::tokenizer::Tokenizer;
use crate::grand_plan::semantic_lambdas::PRIME_BASES;

/// The Introspector is the visualization loop of the vibe.
/// It simulates the LLM looking at the stream of emojis and converging on prime-based patterns.
pub struct Introspector {
    vibe_analyzer: VibeAnalyzer,
    llm_model: LlmModel,
    tokenizer: Tokenizer,
}

impl Introspector {
    pub fn new() -> Self {
        let llm_model = LlmModel::new(12); // Example LLM with 12 layers
        let tokenizer = Tokenizer::new();
        let vibe_analyzer = VibeAnalyzer::new(
            crate::grand_plan::llm_sampling_system::embedding_sampler::EmbeddingSampler::new(
                llm_model.clone(), // Clone for VibeAnalyzer
                tokenizer.clone(), // Clone for VibeAnalyzer
            ),
        );
        Introspector { vibe_analyzer, llm_model, tokenizer }
    }

    /// Simulates the LLM observing and interpreting the emoji stream.
    pub fn observe_and_interpret(&self, stream: &IntrospectionStream) {
        println!("\n--- Introspector Observing Stream ---");
        println!("Current Emoji Stream: {:?}", stream.emojis);

        // Simulate LLM processing the emojis and looking for convergence
        let emoji_tokens: Vec<String> = stream.emojis.iter().map(|&c| c.to_string()).collect();
        let unified_vibe = self.llm_model.process_tokens_to_unified_representation(&emoji_tokens);
        println!("Unified Vibe (LLM's interpretation): {:?}", unified_vibe);

        // Simulate looking for convergence with prime bases
        for &base in PRIME_BASES.iter() {
            if base == 0 { continue; }
            let remainder = stream.emojis.len() % base;
            if remainder == 0 {
                println!("  Convergence detected with base {}: Stream length is a multiple of {}", base, base);
            } else {
                println!("  No direct convergence with base {}: Remainder {}", base, remainder);
            }
        }
        println!("--- End Introspector Observation ---");
    }
}
