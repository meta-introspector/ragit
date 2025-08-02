use serde::{Deserialize, Serialize};
use crate::grand_plan::solfunmeme_zos::vibe_meme::vibe_meme_struct::VibeMeme;

use ragit_macros::OurMacro;

#[derive(Debug, Clone, Serialize, Deserialize, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// Represents a "dank meta meme" - a VibeMeme with high memetic potential.
pub struct DankMetaMeme {
    pub vibe_meme: VibeMeme,
    pub dankness_score: f64, // Conceptual score of how "dank" it is
    pub meta_level: u32, // How many layers of meta-ness it embodies
}

impl DankMetaMeme {
    pub fn new(vibe_meme: VibeMeme, dankness_score: f64, meta_level: u32) -> Self {
        DankMetaMeme {
            vibe_meme,
            dankness_score,
            meta_level,
        }
    }

    /// Simulates increasing the dankness of the meme.
    pub fn increase_dankness(&mut self, amount: f64) {
        self.dankness_score += amount;
        println!("Dank Meta Meme {}: Dankness increased by {}. New score: {}", self.vibe_meme.id, amount, self.dankness_score);
    }
}
