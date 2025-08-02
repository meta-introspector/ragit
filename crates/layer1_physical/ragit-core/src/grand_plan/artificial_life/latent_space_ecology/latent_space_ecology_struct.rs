use crate::grand_plan::artificial_life::artificial_organism::artificial_organism_struct::ArtificialOrganism;
use crate::grand_plan::artificial_life::quasi_mycelium::quasi_mycelium_struct::QuasiMycelium;
use crate::grand_plan::llm_sampling_system::embedding_sampler::embedding_sampler_struct::EmbeddingSampler;
use crate::grand_plan::llm_sampling_system::llm_model::llm_model_struct::LlmModel;
use crate::grand_plan::llm_sampling_system::tokenizer::tokenizer_struct::Tokenizer;

use ragit_macros::OurMacro;

#[derive(OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// Represents the conceptual ecology within the latent space.
pub struct LatentSpaceEcology {
    pub organisms: Vec<ArtificialOrganism>,
    pub mycelium: QuasiMycelium,
    pub embedding_sampler: EmbeddingSampler,
}

impl LatentSpaceEcology {
    pub fn new() -> Self {
        let llm_model = LlmModel::new(12);
        let tokenizer = Tokenizer::new();
        let embedding_sampler = EmbeddingSampler::new(llm_model, tokenizer);
        LatentSpaceEcology {
            organisms: Vec::new(),
            mycelium: QuasiMycelium::new(),
            embedding_sampler,
        }
    }

    /// Simulates a step in the latent space ecology.
    pub fn simulate_step(&mut self) {
        println!("\n--- Latent Space Ecology: Simulating Step ---");

        // Organisms act and potentially reproduce
        let mut new_organisms = Vec::new();
        for organism in &mut self.organisms {
            organism.act();
            if let Some(child) = organism.reproduce() {
                new_organisms.push(child);
            }
        }
        self.organisms.extend(new_organisms);

        // Mycelium grows and connects
        self.mycelium.grow();

        // Conceptual interaction between organisms, mycelium, and latent space
        // (e.g., organisms consume resources from mycelium, mycelium adapts to organism activity)

        println!("--- End Latent Space Ecology Step ---");
    }

    /// Conceptually seeds the ecology with an initial organism based on a string.
    pub fn seed_with_concept(&mut self, concept_string: &str) {
        let tokens = vec![concept_string.to_string()];
        let request = crate::grand_plan::llm_embedding_interface::embedding_request::embedding_request_struct::EmbeddingRequest {
            tokens,
            layer_depths: vec![0], // Get embedding from the first layer
        };
        let response = self.embedding_sampler.sample_embeddings(&request);
        if let Some(embedding) = response.embeddings.get(&0).and_then(|e| e.first()) {
            let organism_id = format!("organism_{}", self.organisms.len());
            let new_organism = ArtificialOrganism::new(organism_id, embedding.clone());
            self.organisms.push(new_organism);
            println!("Seeded ecology with new organism from concept: '{}'", concept_string);
        }
    }
}
