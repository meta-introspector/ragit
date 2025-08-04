
use crate::grand_plan::solana_integration::inference_bid::inference_bid_struct::InferenceBid;
use crate::grand_plan::solana_integration::compute_ask::compute_ask_struct::ComputeAsk;

use ragit_macros::OurMacro;

#[derive(Debug, Default, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// Represents a conceptual marketplace for compute resources.
pub struct ComputeMarketplace {
    pub bids: Vec<InferenceBid>,
    pub asks: Vec<ComputeAsk>,
    // In a real system, this would involve more complex matching algorithms
    // and potentially on-chain order books.
}

impl ComputeMarketplace {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a new inference bid to the marketplace.
    pub fn add_bid(&mut self, bid: InferenceBid) {
        println!("Marketplace: Received new bid: {:?}", bid);
        self.bids.push(bid);
    }

    /// Adds a new compute ask to the marketplace.
    pub fn add_ask(&mut self, ask: ComputeAsk) {
        println!("Marketplace: Received new ask: {:?}", ask);
        self.asks.push(ask);
    }

    /// Simulates matching bids with asks.
    /// Returns a list of (bid, ask) pairs that conceptually match.
    pub fn match_orders(&mut self) -> Vec<(InferenceBid, ComputeAsk)> {
        let mut matched_pairs = Vec::new();
        let mut matched_bid_indices = Vec::new();
        let mut matched_ask_indices = Vec::new();

        // Simple matching logic: find any bid that can be fulfilled by any ask.
        // In a real system, this would be much more sophisticated (e.g., price, capability, quantity).
        for (bid_idx, bid) in self.bids.iter().enumerate() {
            for (ask_idx, ask) in self.asks.iter().enumerate() {
                // Conceptual match: if ask can provide the requested quasifiber type
                if ask.inference_capabilities.contains(&bid.requested_quasifiber_type) &&
                   bid.bid_amount >= ask.price_per_unit {
                    matched_pairs.push((bid.clone(), ask.clone()));
                    matched_bid_indices.push(bid_idx);
                    matched_ask_indices.push(ask_idx);
                    break; // Match this bid with the first suitable ask
                }
            }
        }

        // Remove matched bids and asks (from end to avoid index issues)
        matched_bid_indices.sort_by(|a, b| b.cmp(a));
        for idx in matched_bid_indices { self.bids.remove(idx); }

        matched_ask_indices.sort_by(|a, b| b.cmp(a));
        for idx in matched_ask_indices { self.asks.remove(idx); }

        matched_pairs
    }
}
