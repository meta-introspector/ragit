use std::collections::HashMap;

pub type Uid = usize;

#[derive(Debug, Clone)]
pub struct Leaf<T> {
    pub uid: Uid,
    pub value: T,
}

#[derive(Debug, Clone)]
pub struct BinaryNode {
    pub uid: Uid,
    pub left: Option<Uid>,
    pub right: Option<Uid>,
}

#[derive(Debug)]
pub enum UnitRef<'a, T> {
    Leaf(&'a Leaf<T>),
    Node(&'a BinaryNode),
}

#[derive(Debug, Default)]
pub struct Universe<T> {
    pub leaves: HashMap<Uid, Leaf<T>>,
    pub nodes: HashMap<Uid, BinaryNode>,
    next_uid: Uid,
}

impl<T> Universe<T> {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_next_uid(&mut self) -> Uid {
        let uid = self.next_uid;
        self.next_uid += 1;
        uid
    }

    pub fn new_leaf(&mut self, value: T) -> Uid {
        let uid = self.get_next_uid();
        let leaf = Leaf { uid, value };
        self.leaves.insert(uid, leaf);
        uid
    }

    pub fn new_node(&mut self, left: Option<Uid>, right: Option<Uid>) -> Uid {
        let uid = self.get_next_uid();
        let node = BinaryNode { uid, left, right };
        self.nodes.insert(uid, node);
        uid
    }

    pub fn get(&self, uid: Uid) -> Option<UnitRef<T>> {
        if let Some(leaf) = self.leaves.get(&uid) {
            Some(UnitRef::Leaf(leaf))
        } else if let Some(node) = self.nodes.get(&uid) {
            Some(UnitRef::Node(node))
        } else {
            None
        }
    }
}
