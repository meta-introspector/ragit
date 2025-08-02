use crate::grand_plan::id_indexed_trees::leaf_struct::Leaf;
use crate::grand_plan::id_indexed_trees::node_struct::Node;

use ragit_macros::OurMacro;

#[derive(Debug, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
pub enum UnitRef<'a, T> {
    Leaf(&'a Leaf<T>),
    Node(&'a Node<T>),
}
