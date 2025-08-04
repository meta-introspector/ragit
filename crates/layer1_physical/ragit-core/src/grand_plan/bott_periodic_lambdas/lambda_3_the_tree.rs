use crate::grand_plan::unified_stores::grand_unified_store_struct::GrandUnifiedStore;
use crate::grand_plan::binary_id_trees::universe_struct::Universe;

/// Expands the concept to generate a tree of a slightly larger prime-power size (2^3).
pub fn the_tree(store: &GrandUnifiedStore) -> Option<&Universe<char>> {
    match store.get_store("char").unwrap() {
        crate::grand_plan::unified_stores::type_store_enum::TypeStore::Char(s) => s.get_by_size(8),
        _ => panic!("Invalid type"),
    }
}
