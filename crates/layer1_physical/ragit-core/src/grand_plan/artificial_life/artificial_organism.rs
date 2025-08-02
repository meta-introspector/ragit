use serde::{Deserialize, Serialize};

/// Represents a conceptual artificial organism within the latent space.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtificialOrganism {
    pub id: String,
    pub dna: Vec<f32>, // Conceptual genetic code (e.g., a segment of an embedding)
    pub energy: f64,
    pub age: u64,
    pub behavior_pattern: String, // Simplified: e.g., "explore", "reproduce", "consume"
}

impl ArtificialOrganism {
    pub fn new(id: String, dna: Vec<f32>) -> Self {
        ArtificialOrganism {
            id,
            dna,
            energy: 100.0,
            age: 0,
            behavior_pattern: "explore".to_string(),
        }
    }

    /// Simulates the organism's interaction with its environment.
    pub fn act(&mut self) {
        self.age += 1;
        self.energy -= 1.0; // Consume energy
        println!("Organism {}: Age {}, Energy {}, Behavior: {}", self.id, self.age, self.energy, self.behavior_pattern);
        // In a real simulation, behavior would be determined by DNA and environment.
    }

    /// Simulates reproduction.
    pub fn reproduce(&self) -> Option<ArtificialOrganism> {
        if self.energy > 50.0 {
            let child_dna = self.dna.iter().map(|&x| x + (rand::random::<f32>() - 0.5) * 0.1).collect(); // Simple mutation
            Some(ArtificialOrganism::new(format!("{}_child_{}", self.id, rand::random::<u64>()), child_dna))
        } else {
            None
        }
    }
}
