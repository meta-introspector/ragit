use crate::grand_plan::binary_id_trees::leaf_struct::Leaf;
use crate::grand_plan::binary_id_trees::binary_node_struct::BinaryNode;

use ragit_macros::OurMacro;

#[derive(Debug, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
pub enum UnitRef<'a, T> {
    Leaf(&'a Leaf<T>),
    Node(&'a BinaryNode),
}
