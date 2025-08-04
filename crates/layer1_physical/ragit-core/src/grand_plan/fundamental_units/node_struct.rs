#[derive(Debug, Clone)]
pub struct Node {
    pub id: usize,
    pub children: Vec<crate::grand_plan::fundamental_units::fundamental_unit_enum::FundamentalUnit>,
}
