use crate::grand_plan::trees::node_tree::Node;

#[derive(Debug, Clone, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// The tree unfolds, a fractal dream,
/// Flowing from the generative stream.
pub struct Tree<T>(pub Node<T>);
