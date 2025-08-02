#[derive(Debug, Clone)]
pub enum FundamentalUnit<T> {
    Leaf(T),
    Node(Box<Node<T>>),
}

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub id: usize,
    pub children: Vec<FundamentalUnit<T>>,
}
