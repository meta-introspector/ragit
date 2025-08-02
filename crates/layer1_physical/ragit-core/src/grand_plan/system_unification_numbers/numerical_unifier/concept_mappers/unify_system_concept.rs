use serde::{Deserialize, Serialize};
use crate::grand_plan::system_unification_numbers::numerical_unifier::concept_mappers::mappings::map_base_space::map_base_space;
use crate::grand_plan::system_unification_numbers::numerical_unifier::concept_mappers::mappings::map_inference_space::map_inference_space;
use crate::grand_plan::system_unification_numbers::numerical_unifier::concept_mappers::mappings::map_pair::map_pair;
use crate::grand_plan::system_unification_numbers::numerical_unifier::concept_mappers::mappings::map_tree::map_tree;
use crate::grand_plan::system_unification_numbers::numerical_unifier::concept_mappers::mappings::map_cosmos::map_cosmos;
use crate::grand_plan::system_unification_numbers::numerical_unifier::concept_mappers::mappings::map_osi_layer_7::map_osi_layer_7;
use crate::grand_plan::system_unification_numbers::numerical_unifier::concept_mappers::mappings::map_osi_layer_6::map_osi_layer_6;
use crate::grand_plan::system_unification_numbers::numerical_unifier::concept_mappers::mappings::map_osi_layer_5::map_osi_layer_5;
use crate::grand_plan::system_unification_numbers::numerical_unifier::concept_mappers::mappings::map_osi_layer_4::map_osi_layer_4;
use crate::grand_plan::system_unification_numbers::numerical_unifier::concept_mappers::mappings::map_osi_layer_3::map_osi_layer_3;

use ragit_macros::OurMacro;

#[derive(Debug, Clone, Serialize, Deserialize, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// Represents a conceptual mapping of a system concept to a unifying number.
///
/// This struct associates a named concept with a specific number from the
/// `UNIFYING_NUMBERS` set, along with a descriptive explanation of its meaning
/// within the grand plan.
pub struct UnifiedNumberConcept {
    pub concept_name: String,
    pub unifying_number: u32,
    pub description: String,
}

pub const UNIFYING_NUMBERS: [u32; 10] = [0, 1, 2, 3, 4, 7, 11, 13, 17, 19];

/// Conceptually unifies a system concept (represented by its name) to a number.
///
/// This function attempts to find a corresponding unifying number for a given
/// concept name based on predefined semantic rules. In a real, more advanced
/// system, this mapping could be dynamic, learned, or formally derived.
pub fn unify_system_concept(concept_name: &str) -> Option<UnifiedNumberConcept> {
    println!("Numerical Unifier: Attempting to unify concept: '{}'", concept_name);

    // This is a conceptual mapping. In a real system, this would be based on
    // deeper semantic analysis, formal properties, or configuration.
    match concept_name {
        "BaseSpace" => Some(map_base_space(concept_name)),
        "InferenceSpace" => Some(map_inference_space(concept_name)),
        "Pair" | "Duality" => Some(map_pair(concept_name)),
        "Tree" | "Trinity" => Some(map_tree(concept_name)),
        "Cosmos" | "FourElements" => Some(map_cosmos(concept_name)),
        "OSILayer7" | "Application" => Some(map_osi_layer_7(concept_name)),
        "OSILayer6" | "Presentation" => Some(map_osi_layer_6(concept_name)),
        "OSILayer5" | "Session" => Some(map_osi_layer_5(concept_name)),
        "OSILayer4" | "Transport" => Some(map_osi_layer_4(concept_name)),
        "OSILayer3" | "Network" => Some(map_osi_layer_3(concept_name)),
        _ => None,
    }
}