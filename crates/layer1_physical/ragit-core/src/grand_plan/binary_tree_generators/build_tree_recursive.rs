use crate::grand_plan::id_indexed_trees::uid_type::Uid;
use crate::grand_plan::binary_id_trees::universe_struct::Universe;

/// Recursively builds a balanced binary tree with a specified number of leaves.
pub fn build_tree_recursive<T: Clone + Default>(
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
