use crate::grand_plan::id_indexed_trees::uid_type::Uid;

#[derive(Debug, Clone, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
pub struct Leaf<T> {
    pub uid: Uid,
    pub value: T,
}
