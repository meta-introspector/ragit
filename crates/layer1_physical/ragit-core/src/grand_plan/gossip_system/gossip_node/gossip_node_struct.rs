use crate::grand_plan::gossip_system::gossip_message::gossip_message_enum::GossipMessage;
use crate::grand_plan::introspector_sidechain::introspector_sidechain::introspector_sidechain_struct::IntrospectorSidechain;
use crate::grand_plan::introspector_sidechain::sidechain_event::sidechain_event_enum::SidechainEvent;
use std::collections::HashMap;

use ragit_macros::OurMacro;

#[derive(OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// Represents a single node in the gossip network.
pub struct GossipNode {
    pub id: String,
    pub sidechain: IntrospectorSidechain,
    pub peers: HashMap<String, GossipNode>, // Conceptual: in real life, this would be network addresses
}

impl GossipNode {
    pub fn new(id: String) -> Self {
        GossipNode {
            id,
            sidechain: IntrospectorSidechain::new(),
            peers: HashMap::new(),
        }
    }

    /// Simulates receiving a gossip message.
    pub fn receive_message(&mut self, sender_id: &str, message: GossipMessage) {
        println!("Node {}: Received message from {}: {:?}", self.id, sender_id, message);
        match message {
            GossipMessage::NewIntrospectionEvent(event) => {
                self.sidechain.add_event(event);
                // In a real system, this would trigger mining or propagation
            },
            GossipMessage::RequestHistory(from_block_id) => {
                // Simulate sending back historical events
                let events: Vec<SidechainEvent> = self.sidechain.chain.iter()
                    .skip(from_block_id as usize)
                    .flat_map(|block| block.events.clone())
                    .collect();
                // Conceptual: send this back to the sender
                println!("Node {}: Sending history response to {}", self.id, sender_id);
            },
            GossipMessage::HistoryResponse(events) => {
                // Integrate received history into local sidechain
                println!("Node {}: Integrating received history ({} events)", self.id, events.len());
                for event in events {
                    self.sidechain.add_event(event);
                }
            },
        }
    }

    /// Simulates sending a gossip message to a peer.
    pub fn send_message(&self, receiver_id: &str, message: GossipMessage) {
        println!("Node {}: Sending message to {}: {:?}", self.id, receiver_id, message);
        // In a real system, this would involve network communication.
        // For simulation, we'll just print.
    }

    /// Simulates gossiping a new event to its peers.
    pub fn gossip_event(&self, event: SidechainEvent) {
        for peer_id in self.peers.keys() {
            self.send_message(peer_id, GossipMessage::NewIntrospectionEvent(event.clone()));
        }
    }
}
