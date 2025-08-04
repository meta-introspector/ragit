use crate::grand_plan::id_indexed_trees::uid_type::Uid;

use ragit_macros::OurMacro;

#[derive(Debug, Clone, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
pub struct Leaf<T> {
    pub uid: Uid,
    pub value: T,
}
