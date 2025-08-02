use std::collections::HashMap;
use crate::grand_plan::gossip_system::gossip_node::GossipNode;
use crate::grand_plan::gossip_system::gossip_message::GossipMessage;

/// Represents a conceptual gossip network of nodes.
pub struct GossipNetwork {
    pub nodes: HashMap<String, GossipNode>,
}

impl GossipNetwork {
    pub fn new() -> Self {
        GossipNetwork { nodes: HashMap::new() }
    }

    /// Adds a node to the network.
    pub fn add_node(&mut self, node: GossipNode) {
        self.nodes.insert(node.id.clone(), node);
    }

    /// Simulates a node sending a message to another node in the network.
    pub fn simulate_send_message(&mut self, sender_id: &str, receiver_id: &str, message: GossipMessage) {
        if let Some(receiver_node) = self.nodes.get_mut(receiver_id) {
            receiver_node.receive_message(sender_id, message);
        } else {
            println!("Error: Receiver node {} not found in network.", receiver_id);
        }
    }

    /// Simulates a node gossiping an event to all its known peers.
    pub fn simulate_gossip_event(&mut self, sender_id: &str, event: crate::grand_plan::introspector_sidechain::sidechain_event::SidechainEvent) {
        if let Some(sender_node) = self.nodes.get(sender_id) {
            // Get peers of the sender (conceptual)
            let peers: Vec<String> = sender_node.peers.keys().cloned().collect();
            for peer_id in peers {
                self.simulate_send_message(sender_id, &peer_id, GossipMessage::NewIntrospectionEvent(event.clone()));
            }
        } else {
            println!("Error: Sender node {} not found in network.", sender_id);
        }
    }
}
