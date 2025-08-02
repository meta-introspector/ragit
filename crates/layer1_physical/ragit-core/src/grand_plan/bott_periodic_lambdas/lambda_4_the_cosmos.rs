//use crate::grand_plan::unified_stores::grand_unified_store_struct::{GrandUnifiedStore, TypeStore};
use crate::grand_plan::unified_stores::grand_unified_store_struct::GrandUnifiedStore;
use crate::grand_plan::unified_stores::type_store_enum::TypeStore;

/// A fully populated, sized store for a given type; a universe of possibilities.
pub fn the_cosmos(store: &GrandUnifiedStore) -> &TypeStore {
    store.get_store("i64").unwrap()
}
