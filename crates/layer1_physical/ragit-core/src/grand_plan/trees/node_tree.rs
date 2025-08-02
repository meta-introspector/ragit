#[derive(Debug, Clone, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
pub struct Node<T> {
    pub id: usize,
    pub children: Vec<crate::grand_plan::trees::fundamental_unit_tree::FundamentalUnit<T>>,
}
