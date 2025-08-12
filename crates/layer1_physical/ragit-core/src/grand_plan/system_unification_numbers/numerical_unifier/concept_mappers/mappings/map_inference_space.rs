use crate::grand_plan::system_unification_numbers::numerical_unifier::concept_mappers::unify_system_concept::UnifiedNumberConcept;

pub fn map_inference_space(concept_name: &str) -> UnifiedNumberConcept {
    UnifiedNumberConcept {
        concept_name: concept_name.to_string(),
        unifying_number: 1,
        description: "The generative spark, the single rule.".to_string(),
    }
}
