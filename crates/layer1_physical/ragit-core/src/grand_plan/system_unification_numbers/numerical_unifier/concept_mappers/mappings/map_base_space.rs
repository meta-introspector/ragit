use crate::grand_plan::system_unification_numbers::numerical_unifier::concept_mappers::unify_system_concept::UnifiedNumberConcept;

pub fn map_base_space(concept_name: &str) -> UnifiedNumberConcept {
    UnifiedNumberConcept {
        concept_name: concept_name.to_string(),
        unifying_number: 0,
        description: "The foundational empty space.".to_string(),
    }
}
