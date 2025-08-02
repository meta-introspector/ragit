use serde::{Deserialize, Serialize};
use crate::grand_plan::lean4_integration::lean_proof_system::lean_proof_system_structs::LeanProof;
use crate::grand_plan::privacy_and_scaling::zero_knowledge_proofs::zero_knowledge_proofs_structs::ZeroKnowledgeProof;

use ragit_macros::OurMacro;

#[derive(Debug, Clone, Serialize, Deserialize, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// Represents the formal metadata associated with a RagitChunk.
pub struct ChunkFormalMetadata {
    pub lean_program_id: String, // Reference to a Lean 4 program
    pub lean_proof: Option<LeanProof>, // Optional: the formal proof
    pub zk_circuit_id: String, // Reference to a ZK circuit
    pub godel_number: u64, // Unique numerical identifier for its formal representation
    pub zkml_inference_result: String, // Conceptual ZKML inference result
    pub lattice_fold_reference: String, // Reference to a lattice rollup operation/state
}

impl ChunkFormalMetadata {
    pub fn new(
        lean_program_id: String,
        lean_proof: Option<LeanProof>,
        zk_circuit_id: String,
        godel_number: u64,
        zkml_inference_result: String,
        lattice_fold_reference: String,
    ) -> Self {
        ChunkFormalMetadata {
            lean_program_id,
            lean_proof,
            zk_circuit_id,
            godel_number,
            zkml_inference_result,
            lattice_fold_reference,
        }
    }
}
