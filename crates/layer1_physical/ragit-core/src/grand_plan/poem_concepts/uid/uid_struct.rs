use crate::grand_plan::id_indexed_trees::uid_type::Uid as UidType;

use ragit_macros::OurMacro;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// Assigned a Uid, a time, a place.
pub struct Uid(pub UidType);
