use crate::grand_plan::unified_store::{GrandUnifiedStore, TypeStore};

/// Reflects on the store to retrieve a specific type's universe collection.
pub fn the_mirror<'a>(
    store: &'a GrandUnifiedStore,
    type_name: &str,
) -> Option<&'a TypeStore> {
    store.get_store(type_name)
}
