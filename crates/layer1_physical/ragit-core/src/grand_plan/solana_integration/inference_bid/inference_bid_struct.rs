use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// Represents a bid or request for LLM inference.
pub struct InferenceBid {
    pub requested_quasifiber_type: String,
    pub requested_quasifiber_size: usize,
    pub bid_amount: f64, // Conceptual bid in some unit
    pub callback_address: String, // Where to send the result (e.g., Solana address)
}
