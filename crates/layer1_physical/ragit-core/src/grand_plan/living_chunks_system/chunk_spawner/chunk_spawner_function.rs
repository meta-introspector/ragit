use crate::grand_plan::ragit_chunk_integration::ragit_chunk::ragit_chunk_struct::RagitChunk;
use crate::grand_plan::llm_sampling_system::embedding_sampler::embedding_sampler_struct::EmbeddingSampler;
use crate::grand_plan::llm_sampling_system::llm_model::llm_model_struct::LlmModel;
use crate::grand_plan::llm_sampling_system::tokenizer::tokenizer_struct::Tokenizer;
use crate::grand_plan::solana_integration::solana_program_concept::solana_program_concept_struct::SolanaProgram;
use crate::grand_plan::poem_concepts::quasifiber::quasifiber_struct::Quasifiber;

#[derive(OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// Spawns new RagitChunks based on the output of an execution.
pub fn spawn_new_chunks(
    execution_output: &str,
    spawning_chunk_path: &str,
) -> Vec<RagitChunk> {
    println!("Spawning new chunks from execution output of: {}", spawning_chunk_path);
    let mut new_chunks = Vec::new();

    let llm_model = LlmModel::new(12);
    let tokenizer = Tokenizer::new();
    let embedding_sampler = EmbeddingSampler::new(llm_model, tokenizer);

    // Simulate generating new conceptual modules based on the output
    // In a real system, this would involve LLM generation, code synthesis, etc.
    let new_module_name = format!("generated_module_{:x}", rand::random::<u64>());
    let new_module_path = format!("crates/layer1_physical/ragit-core/src/grand_plan/generated_code/{}.rs", new_module_name);

    // Generate conceptual vibe for the new chunk
    let tokens = embedding_sampler.tokenizer.tokenize_string(&new_module_name);
    let request = crate::grand_plan::llm_embedding_interface::embedding_request::embedding_request_struct::EmbeddingRequest {
        tokens,
        layer_depths: vec![0],
    };
    let response = embedding_sampler.sample_embeddings(&request);
    let conceptual_vibe = response.embeddings.get(&0).unwrap().first().unwrap().clone();

    // Create a dummy Quasifiber for the new SolanaProgram
    let dummy_quasifiber = Quasifiber(crate::grand_plan::binary_id_trees::universe_struct::Universe::new());
    let solana_program: SolanaProgram = dummy_quasifiber.into();

    new_chunks.push(RagitChunk {
        module_path: new_module_path,
        conceptual_vibe,
        solana_program,
        provenance: Some(spawning_chunk_path.to_string()), // Link to the spawning chunk
    });

    new_chunks
}
