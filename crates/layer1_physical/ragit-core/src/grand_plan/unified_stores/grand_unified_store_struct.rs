use std::collections::HashMap;
use crate::grand_plan::unified_stores::type_store_enum::TypeStore;
use crate::grand_plan::sized_universe_stores::generate_sized_power_of_two_trees::generate_sized_power_of_two_trees;

#[derive(Debug, Default, Clone, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// A unified store to hold different types of SizedUniverseStores, indexed by type name.
pub struct GrandUnifiedStore {
    pub stores: HashMap<String, TypeStore>,
}

impl GrandUnifiedStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Generates and adds a store for the `char` type.
    /// The lambda `|i: usize| -> char { (i as u8 % 26 + b'a') as char }` creates the character data.
    pub fn add_char_store(&mut self) {
        let creator = |i: usize| -> char { (i as u8 % 26 + b'a') as char };
        let store = generate_sized_power_of_two_trees(creator);
        self.stores
            .insert("char".to_string(), TypeStore::Char(store));
    }

    /// Generates and adds a store for the `i64` type.
    /// The lambda `|i: usize| -> i64 { i as i64 }` creates the integer data.
    pub fn add_i64_store(&mut self) {
        let creator = |i: usize| -> i64 { i as i64 };
        let store = generate_sized_power_of_two_trees(creator);
        self.stores
            .insert("i64".to_string(), TypeStore::I64(store));
    }

    /// Retrieves a store by its type name.
    pub fn get_store(&self, type_name: &str) -> Option<&TypeStore> {
        self.stores.get(type_name)
    }
}
