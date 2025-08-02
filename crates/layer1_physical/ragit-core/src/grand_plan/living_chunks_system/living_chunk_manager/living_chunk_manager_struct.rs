use crate::grand_plan::ragit_chunk_integration::ragit_chunk::ragit_chunk_struct::RagitChunk;
use crate::grand_plan::living_chunks_system::chunk_executor::chunk_executor_function::execute_chunk;
use crate::grand_plan::living_chunks_system::chunk_spawner::chunk_spawner_function::spawn_new_chunks;
use std::collections::HashMap;

use ragit_macros::OurMacro;

#[derive(OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// Manages the lifecycle of living RagitChunks.
pub struct LivingChunkManager {
    pub active_chunks: HashMap<String, RagitChunk>,
    pub spawned_chunks_history: Vec<RagitChunk>,
}

impl LivingChunkManager {
    pub fn new() -> Self {
        LivingChunkManager {
            active_chunks: HashMap::new(),
            spawned_chunks_history: Vec::new(),
        }
    }

    /// Adds a chunk to the manager's active set.
    pub fn add_active_chunk(&mut self, chunk: RagitChunk) {
        println!("Adding active chunk: {}", chunk.module_path);
        self.active_chunks.insert(chunk.module_path.clone(), chunk);
    }

    /// Simulates a cycle of execution and potential spawning for an active chunk.
    pub fn run_chunk_cycle(&mut self, chunk_path: &str) -> Result<(), String> {
        let chunk = self.active_chunks.get(chunk_path).ok_or_else(|| format!("Chunk not found: {}", chunk_path))?;

        // Execute the chunk
        let output = execute_chunk(chunk);

        // Spawn new chunks based on the output
        let new_chunks = spawn_new_chunks(&output, chunk_path);

        // Add newly spawned chunks to history and potentially to active chunks
        for new_chunk in new_chunks {
            println!("  Spawned new chunk: {}", new_chunk.module_path);
            self.spawned_chunks_history.push(new_chunk.clone());
            // Optionally, add spawned chunks to active_chunks for further cycles
            // self.add_active_chunk(new_chunk);
        }
        Ok(())
    }
}
