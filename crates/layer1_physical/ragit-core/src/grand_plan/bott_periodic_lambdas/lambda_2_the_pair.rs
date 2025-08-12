use crate::grand_plan::unified_stores::grand_unified_store_struct::GrandUnifiedStore;
use crate::grand_plan::binary_id_trees::universe_struct::Universe;

/// Generates the first and simplest universe, a binary tree of size 2^2.
pub fn the_pair(store: &GrandUnifiedStore) -> Option<&Universe<char>> {
    match store.get_store("char").unwrap() {
        crate::grand_plan::unified_stores::type_store_enum::TypeStore::Char(s) => s.get_by_size(4),
        _ => panic!("Invalid type"),
    }
}
