use crate::grand_plan::binary_id_tree::{Uid, Universe};
use std::collections::HashMap;

pub const PRIME_EXPONENTS: [u32; 7] = [2, 3, 5, 7, 11, 17, 19];

/// A store for Universes, indexed by the number of leaves in their trees.
#[derive(Debug)]
pub struct SizedUniverseStore<T> {
    pub universes: HashMap<usize, Universe<T>>,
}

impl<T: Clone> SizedUniverseStore<T> {
    pub fn new() -> Self {
        SizedUniverseStore {
            universes: HashMap::new(),
        }
    }

    /// Retrieves a universe by the number of leaves in its tree.
    pub fn get_by_size(&self, size: usize) -> Option<&Universe<T>> {
        self.universes.get(&size)
    }
}

/// Recursively builds a balanced binary tree with a specified number of leaves.
fn build_tree_recursive<T: Clone>(
    universe: &mut Universe<T>,
    creator: &impl Fn(usize) -> T,
    leaf_counter: &mut usize,
    leaves_to_create: usize,
) -> Option<Uid> {
    if leaves_to_create == 0 {
        return None;
    }

    if leaves_to_create == 1 {
        let value = creator(*leaf_counter);
        *leaf_counter += 1;
        return Some(universe.new_leaf(value));
    }

    let left_size = leaves_to_create / 2;
    let right_size = leaves_to_create - left_size;

    let left_child_uid = build_tree_recursive(universe, creator, leaf_counter, left_size);
    let right_child_uid = build_tree_recursive(universe, creator, leaf_counter, right_size);

    Some(universe.new_node(left_child_uid, right_child_uid))
}

/// Generates a SizedUniverseStore containing binary trees where the number of leaves
/// is 2 raised to a prime exponent.
pub fn generate_sized_power_of_two_trees<T: Clone>(
    creator: impl Fn(usize) -> T,
) -> SizedUniverseStore<T> {
    let mut store = SizedUniverseStore::new();

    for &power in PRIME_EXPONENTS.iter() {
        let mut universe = Universe::new();
        let num_leaves = 1_usize.pow(power);
        let mut leaf_counter = 0;

        build_tree_recursive(&mut universe, &creator, &mut leaf_counter, num_leaves);
        store.universes.insert(num_leaves, universe);
    }

    store
}
