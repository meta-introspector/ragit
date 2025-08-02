use serde::{Deserialize, Serialize};
use crate::grand_plan::meme_economy::dank_meta_meme::dank_meta_meme_struct::DankMetaMeme;
use crate::grand_plan::solfunmeme_zos::hyper_pump_mechanism::hyper_pump_mechanism_struct::HyperPumpMechanism;
use crate::grand_plan::solfunmeme_zos::solfunmeme_core::solfunmeme_core_struct::Solfunmeme;

#[derive(Debug, Clone, Serialize, Deserialize, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// Represents a Meme Lord, an agent who invests in and punts dank meta memes.
pub struct MemeLord {
    pub id: String,
    pub capital: f64, // Conceptual capital for investment
    pub portfolio: Vec<DankMetaMeme>,
}

impl MemeLord {
    pub fn new(id: String, initial_capital: f64) -> Self {
        MemeLord { id, capital: initial_capital, portfolio: Vec::new() }
    }

    /// Simulates a Meme Lord investing in a dank meta meme.
    pub fn invest_in_meme(&mut self, meme: DankMetaMeme, investment_amount: f64) -> Result<(), String> {
        if self.capital < investment_amount {
            return Err("Insufficient capital to invest.".to_string());
        }
        self.capital -= investment_amount;
        self.portfolio.push(meme);
        println!("Meme Lord {}: Invested {} in meme {}. Remaining capital: {}", self.id, investment_amount, self.portfolio.last().unwrap().vibe_meme.id, self.capital);
        Ok(())
    }

    /// Simulates a Meme Lord "punting" a meme to activate a hype cycle.
    pub fn punt_meme_to_hype_cycle(&mut self, meme_id: &str, solfunmeme_system: &mut Solfunmeme, hyper_pump: &HyperPumpMechanism) -> Result<(), String> {
        if let Some(meme) = self.portfolio.iter_mut().find(|m| m.vibe_meme.id == meme_id) {
            println!("Meme Lord {}: Punting meme {} to hype cycle.", self.id, meme_id);
            let new_energy = hyper_pump.activate_pump(solfunmeme_system.memetic_energy);
            solfunmeme_system.memetic_energy = new_energy;
            meme.vibe_meme.adjust_energy(new_energy * 0.1); // Meme gains some energy from the pump
            Ok(())
        } else {
            Err(format!("Meme {} not found in Meme Lord's portfolio.", meme_id))
        }
    }
}
