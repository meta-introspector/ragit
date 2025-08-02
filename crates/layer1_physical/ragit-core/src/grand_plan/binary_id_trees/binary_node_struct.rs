use crate::grand_plan::id_indexed_trees::uid_type::Uid;

#[derive(Debug, Clone, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
pub struct BinaryNode {
    pub uid: Uid,
    pub left: Option<Uid>,
    pub right: Option<Uid>,
}
