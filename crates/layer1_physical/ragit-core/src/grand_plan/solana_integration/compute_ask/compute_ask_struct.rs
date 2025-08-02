use serde::{Deserialize, Serialize};

use ragit_macros::OurMacro;

#[derive(Debug, Clone, Serialize, Deserialize, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// Represents a provider's offer for computational resources (an "ask").
pub struct ComputeAsk {
    pub provider_id: String, // Unique ID of the compute provider
    pub resource_type: String, // e.g., "CPU", "GPU", "Memory", "Credits"
    pub available_quantity: f64, // Amount of resource available
    pub price_per_unit: f64, // Price for the resource
    pub inference_capabilities: Vec<String>, // Types of inference this provider can perform
    pub provider_address: String, // Solana address of the provider
}
