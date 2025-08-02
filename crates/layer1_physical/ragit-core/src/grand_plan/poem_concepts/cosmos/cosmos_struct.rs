use crate::grand_plan::sized_universe_stores::sized_universe_store_struct::SizedUniverseStore;

use ragit_macros::OurMacro;

#[derive(Debug, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// At 11, a cosmos, full and vast,
/// A SizedStore built, a mold is cast.
pub struct Cosmos<T>(pub SizedUniverseStore<T>);
