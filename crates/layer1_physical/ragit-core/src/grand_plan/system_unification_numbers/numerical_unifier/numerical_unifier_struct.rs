use serde::{Deserialize, Serialize};
use crate::grand_plan::unified_concept_enum::concept_enum::unified_concept_enum_enum::UnifiedConceptEnum;
use crate::grand_plan::system_unification_numbers::numerical_unifier::concept_mappers::unify_system_concept::{UnifiedNumberConcept, UNIFYING_NUMBERS};
use crate::grand_plan::system_unification_numbers::numerical_unifier::unified_concept_enum_unifier::unify_unified_concept_enum::unify_unified_concept_enum;

pub struct NumericalUnifier;

impl NumericalUnifier {
    /// Creates a new instance of `NumericalUnifier`.
    pub fn new() -> Self { NumericalUnifier {} }

    /// Conceptually unifies a system concept (represented by its name) to a number.
    ///
    /// This function attempts to find a corresponding unifying number for a given
    /// concept name based on predefined semantic rules. In a real, more advanced
    /// system, this mapping could be dynamic, learned, or formally derived.
    pub fn unify_system_concept(&self, concept_name: &str) -> Option<UnifiedNumberConcept> {
        unify_system_concept(concept_name)
    }

    /// Conceptually unifies a `UnifiedConceptEnum` to a number.
    ///
    /// This method extracts the conceptual name from a `UnifiedConceptEnum`
    /// variant and attempts to map it to a unifying number using `unify_system_concept`.
    pub fn unify_unified_concept_enum(&self, concept: &UnifiedConceptEnum) -> Option<UnifiedNumberConcept> {
        unify_unified_concept_enum(concept)
    }
}