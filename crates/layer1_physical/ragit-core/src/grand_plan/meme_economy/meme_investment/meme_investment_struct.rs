use serde::{Deserialize, Serialize};

use ragit_macros::OurMacro;

#[derive(Debug, Clone, Serialize, Deserialize, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// Represents a conceptual investment in a meme.
pub struct MemeInvestment {
    pub investor_id: String,
    pub meme_id: String,
    pub amount: f64,
    pub timestamp: u64,
}
