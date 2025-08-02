use ragit_macros::OurMacro;

#[derive(Debug, Clone, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
pub enum FundamentalUnit<T> {
    Leaf(T),
    Node(Box<crate::grand_plan::trees::node_tree::Node<T>>),
}
