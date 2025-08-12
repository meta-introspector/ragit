use crate::grand_plan::unified_stores::grand_unified_store_struct::GrandUnifiedStore;
use crate::grand_plan::binary_id_trees::universe_struct::Universe;

/// Pulls a single, concrete instance (a tree of size n) from the store.
pub fn the_quasifiber<'a, T>(
    store: &'a GrandUnifiedStore,
    type_name: &str,
    size: usize,
) -> &'a Universe<T> {
    // This is a simplified example. A real implementation would need to handle the generic T.
    // For now, we'll assume the caller knows the type.
    match store.get_store(type_name).unwrap() {
        crate::grand_plan::unified_stores::type_store_enum::TypeStore::Char(s) => unsafe {
            &*(s.get_by_size(size).unwrap() as *const _ as *const Universe<T>)
        },
        crate::grand_plan::unified_stores::type_store_enum::TypeStore::I64(s) => unsafe {
            &*(s.get_by_size(size).unwrap() as *const _ as *const Universe<T>)
        },
    }
}
