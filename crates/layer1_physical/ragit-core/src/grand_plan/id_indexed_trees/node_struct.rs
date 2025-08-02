use crate::grand_plan::id_indexed_trees::uid_type::Uid;

#[derive(Debug, Clone, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
pub struct Node<T> {
    pub uid: Uid,
    pub children: Vec<Uid>,
}
