use serde::{Deserialize, Serialize};

/// Represents a Vibe (embedding vector) as a Meme within the SOLFUNMEME system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VibeMeme {
    pub id: String,
    pub vibe_vector: Vec<f32>,
    pub memetic_energy: f64,
    pub virality_score: f64,
    pub narrative_tags: Vec<String>,
}

impl VibeMeme {
    pub fn new(id: String, vibe_vector: Vec<f32>) -> Self {
        VibeMeme {
            id,
            vibe_vector,
            memetic_energy: 100.0, // Initial energy
            virality_score: 0.1,  // Initial virality
            narrative_tags: Vec::new(),
        }
    }

    /// Simulates the evolution of the VibeMeme.
    pub fn evolve(&mut self, new_vibe_vector: Vec<f32>, new_tags: Vec<String>) {
        println!("VibeMeme {}: Evolving...", self.id);
        self.vibe_vector = new_vibe_vector;
        self.narrative_tags.extend(new_tags);
        self.memetic_energy *= 1.05; // Conceptual growth
        self.virality_score *= 1.02; // Conceptual virality increase
    }

    /// Simulates the VibeMeme gaining or losing memetic energy.
    pub fn adjust_energy(&mut self, amount: f64) {
        self.memetic_energy += amount;
        println!("VibeMeme {}: Energy adjusted by {}. New energy: {}", self.id, amount, self.memetic_energy);
    }
}
