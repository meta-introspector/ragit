use crate::grand_plan::unified_concept_enum::concept_enum::unified_concept_enum_enum::UnifiedConceptEnum;
// use crate::grand_plan::system_unification_numbers::numerical_unifier::concept_mappers::unify_system_concept::{unify_system_concept, UnifiedNumberConcept};
use crate::grand_plan::system_unification_numbers::numerical_unifier::concept_mappers::unify_system_concept::UnifiedNumberConcept;

/// Conceptually unifies a `UnifiedConceptEnum` to a number.
///
/// This method extracts the conceptual name from a `UnifiedConceptEnum`
/// variant and attempts to map it to a unifying number using `unify_system_concept`.
pub fn unify_unified_concept_enum(concept: &UnifiedConceptEnum) -> Option<UnifiedNumberConcept> {
    let concept_name = format!("{:?}", concept).split('(').next().unwrap().to_string();
    // unify_system_concept(&concept_name)
    None // Temporarily return None
}
