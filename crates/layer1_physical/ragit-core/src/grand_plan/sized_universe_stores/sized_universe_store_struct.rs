use std::collections::HashMap;
use crate::grand_plan::binary_id_trees::universe_struct::Universe;

#[derive(Debug, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// A store for Universes, indexed by the number of leaves in their trees.
pub struct SizedUniverseStore<T> {
    pub universes: HashMap<usize, Universe<T>>,
}

impl<T: Clone> SizedUniverseStore<T> {
    pub fn new() -> Self {
        SizedUniverseStore {
            universes: HashMap::new(),
        }
    }

    /// Retrieves a universe by the number of leaves in its tree.
    pub fn get_by_size(&self, size: usize) -> Option<&Universe<T>> {
        self.universes.get(&size)
    }
}
