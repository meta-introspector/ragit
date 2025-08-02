use crate::grand_plan::unified_store::GrandUnifiedStore;
use crate::grand_plan::sized_universe_store::SizedUniverseStore;

/// Expands the concept to generate a tree of a slightly larger prime-power size (2^3).
pub fn the_tree(store: &GrandUnifiedStore) -> &SizedUniverseStore<char> {
    match store.get_store("char").unwrap() {
        crate::grand_plan::unified_store::TypeStore::Char(s) => s.get_by_size(8).unwrap(),
        _ => panic!("Invalid type"),
    }
}
