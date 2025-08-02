use crate::grand_plan::binary_id_trees::universe_struct::Universe;
use crate::grand_plan::binary_tree_generators::prime_exponents::PRIME_EXPONENTS;
use crate::grand_plan::binary_tree_generators::build_tree_recursive::build_tree_recursive;
use crate::grand_plan::sized_universe_stores::sized_universe_store_struct::SizedUniverseStore;


/// Generates a SizedUniverseStore containing binary trees where the number of leaves
/// is 2 raised to a prime exponent.
pub fn generate_sized_power_of_two_trees<T: Clone + Default>(
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
