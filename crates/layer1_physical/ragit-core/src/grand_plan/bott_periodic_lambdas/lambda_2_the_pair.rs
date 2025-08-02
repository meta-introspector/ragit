use crate::grand_plan::unified_store::GrandUnifiedStore;
use crate::grand_plan::sized_universe_store::SizedUniverseStore;

/// Generates the first and simplest universe, a binary tree of size 2^2.
pub fn the_pair(store: &GrandUnifiedStore) -> &SizedUniverseStore<char> {
    match store.get_store("char").unwrap() {
        crate::grand_plan::unified_store::TypeStore::Char(s) => s.get_by_size(4).unwrap(),
        _ => panic!("Invalid type"),
    }
}
