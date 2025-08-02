use serde::{Deserialize, Serialize};

/// Represents a single event or transaction on the Introspector Sidechain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SidechainEvent {
    /// An LLM operation occurred.
    LlmOperation(String), // Simplified: just the operation description
    /// An emoji stream was observed.
    EmojiStreamObservation(Vec<char>),
    /// A vibe function was executed.
    VibeFunctionExecution { name: String, result: String },
    /// A new UnifiedConcept was formed.
    UnifiedConceptFormation(String), // Simplified: concept description
    // Add more event types as needed
}
