use serde::{Deserialize, Serialize};

use ragit_macros::OurMacro;

#[derive(Debug, Clone, Serialize, Deserialize, OurMacro)]
pub struct ZeroKnowledgeProof {
    // This is a placeholder for a real ZKP implementation.
    // In a real system, this would contain the proof data.
    pub proof_data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, OurMacro)]
pub struct VerifiableData<T> {
    pub data: T,
    pub proof: ZeroKnowledgeProof,
}
