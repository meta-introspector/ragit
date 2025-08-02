use crate::grand_plan::introspector_sidechain::sidechain_block::sidechain_block_struct::SidechainBlock;
use crate::grand_plan::introspector_sidechain::sidechain_event::sidechain_event_enum::SidechainEvent;
use std::time::{SystemTime, UNIX_EPOCH};

use ragit_macros::OurMacro;

#[derive(OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// Represents the Introspector Sidechain (a conceptual blockchain).
pub struct IntrospectorSidechain {
    pub chain: Vec<SidechainBlock>,
    pub pending_events: Vec<SidechainEvent>,
}

impl IntrospectorSidechain {
    pub fn new() -> Self {
        let mut chain = Vec::new();
        // Create the genesis block
        chain.push(SidechainBlock::new(
            0,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            vec![SidechainEvent::UnifiedConceptFormation(
                "Genesis Block Created".to_string(),
            )],
            "0".to_string(), // Genesis block has no previous hash
        ));
        IntrospectorSidechain { chain, pending_events: Vec::new() }
    }

    /// Adds a new event to the pending events list.
    pub fn add_event(&mut self, event: SidechainEvent) {
        self.pending_events.push(event);
    }

    /// Mines a new block, adding all pending events to it.
    pub fn mine_block(&mut self) -> &SidechainBlock {
        let last_block = self.chain.last().unwrap();
        let new_block_id = last_block.id + 1;
        let new_block_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let new_block_previous_hash = last_block.hash.clone();
        let new_block_events = self.pending_events.drain(..).collect();

        let new_block = SidechainBlock::new(
            new_block_id,
            new_block_timestamp,
            new_block_events,
            new_block_previous_hash,
        );
        self.chain.push(new_block);
        self.chain.last().unwrap()
    }

    /// Retrieves a block by its ID.
    pub fn get_block(&self, id: u64) -> Option<&SidechainBlock> {
        self.chain.get(id as usize)
    }
}
