use crate::grand_plan::ragit_chunk_integration::ragit_chunk::ragit_chunk_struct::RagitChunk;
use crate::grand_plan::living_chunks_system::chunk_spawner::initializers::initialize_llm_components::initialize_llm_components;
use crate::grand_plan::living_chunks_system::chunk_spawner::chunk_creation::create_new_module::create_new_module;
use crate::grand_plan::living_chunks_system::chunk_spawner::chunk_creation::create_solana_program::create_solana_program;
use crate::grand_plan::living_chunks_system::chunk_spawner::metadata_generation::generate_formal_metadata::generate_formal_metadata;

pub fn spawn_new_chunks(
    _execution_output: &str,
    spawning_chunk_path: &str,
) -> Vec<RagitChunk> {
    println!("Spawning new chunks from execution output of: {}", spawning_chunk_path);
    let mut new_chunks = Vec::new();

    let llm_components = initialize_llm_components();

    // Simulate generating new conceptual modules based on the output
    // In a real system, this would involve LLM generation, code synthesis, etc.
    let new_module_name = format!("generated_module_{:x}", rand::random::<u64>());
    let (new_module_name, new_module_path, conceptual_vibe) = create_new_module(&llm_components.embedding_sampler, &new_module_name);

    let solana_program = create_solana_program();

    let formal_metadata = generate_formal_metadata(&new_module_name);

    new_chunks.push(RagitChunk {
        module_path: new_module_path,
        conceptual_vibe,
        solana_program,
        provenance: Some(spawning_chunk_path.to_string()), // Link to the spawning chunk
        formal_metadata,
    });

    new_chunks
}