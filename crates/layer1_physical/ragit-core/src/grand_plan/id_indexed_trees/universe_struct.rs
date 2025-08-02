use std::collections::HashMap;
use crate::grand_plan::id_indexed_trees::uid_type::Uid;
use crate::grand_plan::id_indexed_trees::leaf_struct::Leaf;
use crate::grand_plan::id_indexed_trees::node_struct::Node;
use crate::grand_plan::id_indexed_trees::unit_ref_enum::UnitRef;

#[derive(Debug, Default, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
pub struct Universe<T> {
    pub leaves: HashMap<Uid, Leaf<T>>,
    pub nodes: HashMap<Uid, Node<T>>,
    next_uid: Uid,
}

impl<T> Universe<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_next_uid(&mut self) -> Uid {
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

    pub fn new_node(&mut self, children: Vec<Uid>) -> Uid {
        let uid = self.get_next_uid();
        let node = Node { uid, children };
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
