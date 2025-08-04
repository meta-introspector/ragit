#[derive(Debug, Clone)]
pub enum FundamentalUnit {
    Char(char),
    Int(i64),
    Vector(Vec<FundamentalUnit>),
    Node(Box<crate::grand_plan::fundamental_units::node_struct::Node>),
}
