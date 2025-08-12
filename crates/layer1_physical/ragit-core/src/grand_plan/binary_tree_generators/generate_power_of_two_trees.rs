use crate::grand_plan::binary_id_trees::universe_struct::Universe;
use crate::grand_plan::binary_tree_generators::prime_exponents::PRIME_EXPONENTS;
use crate::grand_plan::binary_tree_generators::build_tree_recursive::build_tree_recursive;


/// Generates a vector of Universes, each containing a binary tree
/// with a number of leaves equal to 2 raised to a prime exponent.
pub fn generate_power_of_two_trees<T: Clone + Default>(creator: impl Fn(usize) -> T) -> Vec<Universe<T>> {
    let mut universes = Vec::new();

    for &power in PRIME_EXPONENTS.iter() {
        let mut universe = Universe::new();
        let num_leaves = 1_usize.pow(power);
        let mut leaf_counter = 0;

        build_tree_recursive(&mut universe, &creator, &mut leaf_counter, num_leaves);
        universes.push(universe);
    }

    universes
}
