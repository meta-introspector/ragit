use serde::{Deserialize, Serialize};

/// Represents a conceptual investment in a meme.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemeInvestment {
    pub investor_id: String,
    pub meme_id: String,
    pub amount: f64,
    pub timestamp: u64,
}
