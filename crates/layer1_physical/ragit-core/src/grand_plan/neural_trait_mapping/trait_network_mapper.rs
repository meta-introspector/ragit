use std::collections::HashMap;
use crate::grand_plan::solfunmeme_zos::meme_traits::MemeTrait;
use crate::grand_plan::neural_trait_mapping::neural_trait::NeuralTrait;

/// Maps MemeTraits to NeuralTraits, giving them intrinsic meaning.
pub struct TraitNetworkMapper {
    pub mappings: HashMap<String, NeuralTrait>,
}

impl TraitNetworkMapper {
    pub fn new() -> Self {
        TraitNetworkMapper { mappings: HashMap::new() }
    }

    /// Conceptually maps a MemeTrait to a NeuralTrait.
    pub fn map_meme_trait_to_neural_trait(&mut self, meme_trait: &MemeTrait) {
        println!("Mapping MemeTrait '{}' to NeuralTrait.", meme_trait.name);
        let neural_trait = match meme_trait.name.as_str() {
            "Eb" => NeuralTrait::Eigenvector(vec![0.8, 0.2, 0.1]), // Blue Eye as an eigenvector
            "Pr" => NeuralTrait::SelfGeneratingNetwork { 
                network_id: "chaotic_growth_net".to_string(),
                generation_seed: 123,
                complexity_score: 0.9,
            }, // Red Petals as a self-generating network
            // Example for intrinsic meaning of numbers like 2
            "My" => NeuralTrait::Eigenvector(vec![0.5, 0.5]), // Mycelium, representing duality (2)
            _ => NeuralTrait::Eigenvector(vec![meme_trait.initial_value as f32]), // Default to simple eigenvector
        };
        self.mappings.insert(meme_trait.name.clone(), neural_trait);
    }

    /// Retrieves the NeuralTrait for a given MemeTrait name.
    pub fn get_neural_trait(&self, trait_name: &str) -> Option<&NeuralTrait> {
        self.mappings.get(trait_name)
    }
}
