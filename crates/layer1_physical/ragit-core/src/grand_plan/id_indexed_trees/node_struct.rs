use crate::grand_plan::id_indexed_trees::uid_type::Uid;

use ragit_macros::OurMacro;

#[derive(Debug, Clone, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
pub struct Node {
    pub uid: Uid,
    pub children: Vec<Uid>,
}
