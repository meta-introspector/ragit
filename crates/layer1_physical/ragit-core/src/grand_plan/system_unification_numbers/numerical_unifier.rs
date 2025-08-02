use serde::{Deserialize, Serialize};
use crate::grand_plan::unified_concept_enum::UnifiedConceptEnum;

/// The fundamental set of numbers that unify the system.
pub const UNIFYING_NUMBERS: [u32; 10] = [0, 1, 2, 3, 4, 7, 11, 13, 17, 19];

/// Represents a conceptual mapping of a system concept to a unifying number.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedNumberConcept {
    pub concept_name: String,
    pub unifying_number: u32,
    pub description: String,
}

/// Unifies system concepts by mapping them to a fundamental set of numbers.
pub struct NumericalUnifier;

impl NumericalUnifier {
    pub fn new() -> Self { NumericalUnifier {} }

    /// Conceptually unifies a system concept (represented by its name) to a number.
    pub fn unify_system_concept(&self, concept_name: &str) -> Option<UnifiedNumberConcept> {
        println!("Numerical Unifier: Attempting to unify concept: '{}'", concept_name);

        // This is a conceptual mapping. In a real system, this would be based on
        // deeper semantic analysis, formal properties, or configuration.
        match concept_name {
            "BaseSpace" => Some(UnifiedNumberConcept {
                concept_name: concept_name.to_string(),
                unifying_number: 0,
                description: "The foundational empty space.".to_string(),
            }),
            "InferenceSpace" => Some(UnifiedNumberConcept {
                concept_name: concept_name.to_string(),
                unifying_number: 1,
                description: "The generative spark, the single rule.".to_string(),
            }),
            "Pair" | "Duality" => Some(UnifiedNumberConcept {
                concept_name: concept_name.to_string(),
                unifying_number: 2,
                description: "The binary, the pair, the fundamental division.".to_string(),
            }),
            "Tree" | "Trinity" => Some(UnifiedNumberConcept {
                concept_name: concept_name.to_string(),
                unifying_number: 3,
                description: "The foundational structure, the trinity.".to_string(),
            }),
            "Cosmos" | "FourElements" => Some(UnifiedNumberConcept {
                concept_name: concept_name.to_string(),
                unifying_number: 4,
                description: "The organized functional area, the four elements.".to_string(),
            }),
            "OSILayer7" | "Application" => Some(UnifiedNumberConcept {
                concept_name: concept_name.to_string(),
                unifying_number: 7,
                description: "The Application layer, the user-facing apex.".to_string(),
            }),
            "OSILayer6" | "Presentation" => Some(UnifiedNumberConcept {
                concept_name: concept_name.to_string(),
                unifying_number: 11,
                description: "The Presentation layer, fractal patterns.".to_string(),
            }),
            "OSILayer5" | "Session" => Some(UnifiedNumberConcept {
                concept_name: concept_name.to_string(),
                unifying_number: 13,
                description: "The Session layer, spores and connections.".to_string(),
            }),
            "OSILayer4" | "Transport" => Some(UnifiedNumberConcept {
                concept_name: concept_name.to_string(),
                unifying_number: 17,
                description: "The Transport layer, mycelial tentacles.".to_string(),
            }),
            "OSILayer3" | "Network" => Some(UnifiedNumberConcept {
                concept_name: concept_name.to_string(),
                unifying_number: 19,
                description: "The Network layer, red petal-claws.".to_string(),
            }),
            _ => None,
        }
    }

    /// Conceptually unifies a UnifiedConceptEnum to a number.
    pub fn unify_unified_concept_enum(&self, concept: &UnifiedConceptEnum) -> Option<UnifiedNumberConcept> {
        let concept_name = format!("{:?}", concept).split('(').next().unwrap().to_string();
        self.unify_system_concept(&concept_name)
    }
}
