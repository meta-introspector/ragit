use crate::grand_plan::unified_stores::grand_unified_store_struct::GrandUnifiedStore;

use ragit_macros::OurMacro;

#[derive(Debug, Clone, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// The Base Space waits, a silent plea,
/// With sizes etched: 2, 3, 5, for all to see.
pub struct BaseSpace(pub GrandUnifiedStore);
