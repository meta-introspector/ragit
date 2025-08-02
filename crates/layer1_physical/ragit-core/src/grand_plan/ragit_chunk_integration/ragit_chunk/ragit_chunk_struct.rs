use serde::{Deserialize, Serialize};
use crate::grand_plan::solana_integration::solana_program_concept::solana_program_concept_struct::SolanaProgram;
use crate::grand_plan::ragit_chunk_integration::chunk_formal_metadata::chunk_formal_metadata_struct::ChunkFormalMetadata;

use ragit_macros::OurMacro;

#[derive(Debug, Clone, Serialize, Deserialize, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// Represents a conceptual ragit chunk, derived from a Rust module.
pub struct RagitChunk {
    pub module_path: String,
    pub conceptual_vibe: Vec<f32>, // The embedding of the module's content/concept
    pub solana_program: SolanaProgram,
    pub provenance: Option<String>, // Link to the chunk/code that spawned this one
    pub formal_metadata: ChunkFormalMetadata,
}
