use crate::grand_plan::unified_store::GrandUnifiedStore;
use crate::grand_plan::binary_id_tree::Universe;

/// The complete, unified operation: generating and retrieving a specific structure on demand.
pub fn the_cycle(type_name: &str, size: usize) -> GrandUnifiedStore {
    let mut s = GrandUnifiedStore::new();
    s.add_char_store();
    s.add_i64_store();
    s
}
