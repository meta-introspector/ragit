#[derive(Debug, Clone)]
pub enum FundamentalUnit {
    Char(char),
    Int(i64),
    Vector(Vec<FundamentalUnit>),
    Node(Box<Node>),
}

#[derive(Debug, Clone)]
pub struct Node {
    // Placeholder for now
    pub id: usize,
    pub children: Vec<FundamentalUnit>,
}
