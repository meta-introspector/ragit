use crate::grand_plan::introspector_sidechain::sidechain_event::sidechain_event_enum::SidechainEvent;
use serde::{Deserialize, Serialize};
use fnv::FnvHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Serialize, Deserialize, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// Represents a block in the Introspector Sidechain.
pub struct SidechainBlock {
    pub id: u64,
    pub timestamp: u64,
    pub events: Vec<SidechainEvent>,
    pub previous_hash: String,
    pub hash: String,
}

impl SidechainBlock {
    /// Creates a new block.
    pub fn new(
        id: u64,
        timestamp: u64,
        events: Vec<SidechainEvent>,
        previous_hash: String,
    ) -> Self {
        let mut block = SidechainBlock {
            id,
            timestamp,
            events,
            previous_hash,
            hash: String::new(), // Will be calculated after creation
        };
        block.hash = block.calculate_hash();
        block
    }

    /// Calculates the hash of the block.
    fn calculate_hash(&self) -> String {
        let mut hasher = FnvHasher::default();
        self.id.hash(&mut hasher);
        self.timestamp.hash(&mut hasher);
        self.previous_hash.hash(&mut hasher);
        // Hash the events by their debug representation for simplicity
        format!("{:x}", hasher.finish() ^ format!("{:?}", self.events).as_bytes().iter().fold(0u64, |acc, &b| acc.rotate_left(5).wrapping_add(b as u64)))
    }
}
