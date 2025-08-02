use crate::grand_plan::unified_store::{GrandUnifiedStore, TypeStore};

/// A fully populated, sized store for a given type; a universe of possibilities.
pub fn the_cosmos(store: &GrandUnifiedStore) -> &TypeStore {
    store.get_store("i64").unwrap()
}
