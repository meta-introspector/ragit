use crate::grand_plan::binary_id_trees::universe_struct::Universe;

use ragit_macros::OurMacro;

#[derive(Debug, Clone, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// A Quasifiber from the pattern bred.
/// A tree of size n, a concrete prize,
pub struct Quasifiber<T>(pub Universe<T>);
