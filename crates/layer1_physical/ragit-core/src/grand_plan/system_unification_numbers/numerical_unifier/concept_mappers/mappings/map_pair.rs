use crate::grand_plan::system_unification_numbers::numerical_unifier::concept_mappers::unify_system_concept::UnifiedNumberConcept;

pub fn map_pair(concept_name: &str) -> UnifiedNumberConcept {
    UnifiedNumberConcept {
        concept_name: concept_name.to_string(),
        unifying_number: 2,
        description: "The binary, the pair, the fundamental division.".to_string(),
    }
}
