use serde::{Deserialize, Serialize};
use crate::grand_plan::introspector_sidechain::sidechain_event::sidechain_event_enum::SidechainEvent;

#[derive(Debug, Clone, Serialize, Deserialize, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// Represents a message exchanged in the gossip network.
pub enum GossipMessage {
    /// A new introspection event observed by a node.
    NewIntrospectionEvent(SidechainEvent),
    /// A request for historical events from another node.
    RequestHistory(u64), // Request events from a certain block ID onwards
    /// A response containing historical events.
    HistoryResponse(Vec<SidechainEvent>),
    // Add more message types as needed (e.g., peer discovery, health checks)
}
