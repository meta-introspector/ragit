#[derive(Debug, Clone)]
pub struct FundamentalUnit<T> {
    pub value: T,
}

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub id: usize,
    pub children: Vec<FundamentalUnit<T>>,
}
