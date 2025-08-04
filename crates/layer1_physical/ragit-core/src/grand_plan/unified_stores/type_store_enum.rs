use crate::grand_plan::sized_universe_stores::sized_universe_store_struct::SizedUniverseStore;

use ragit_macros::OurMacro;

#[derive(Debug, Clone, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// An enum to hold a SizedUniverseStore of a specific type.
pub enum TypeStore {
    Char(SizedUniverseStore<char>),
    I64(SizedUniverseStore<i64>),
    // Add other types here as needed
}
