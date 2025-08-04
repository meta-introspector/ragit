use ragit_macros::OurMacro;

#[derive(Debug, Clone, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
pub struct Node<T> {
    pub id: usize,
    pub children: Vec<crate::grand_plan::generic_units::fundamental_unit_generic::FundamentalUnit<T>>,
}