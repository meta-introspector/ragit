use crate::grand_plan::unified_store::GrandUnifiedStore;

/// Creates the empty, unified store; the base space.
pub fn the_void() -> GrandUnifiedStore {
    GrandUnifiedStore::new()
}
