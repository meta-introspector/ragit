use crate::grand_plan::id_indexed_trees::uid_type::Uid;

use ragit_macros::OurMacro;

#[derive(Debug, Clone, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
pub struct BinaryNode {
    pub uid: Uid,
    pub left: Option<Uid>,
    pub right: Option<Uid>,
}
