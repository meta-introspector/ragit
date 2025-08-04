use serde::{Deserialize, Serialize};

use ragit_macros::OurMacro;

#[derive(Debug, Clone, Serialize, Deserialize, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// Represents a generic value that can be passed across the Lean ABI.
pub enum LeanValue {
    U32(u32),
    String(String),
    Bool(bool),
    // Add more types as needed
}

/// Represents the arguments passed to a Lean ABI function.
pub type LeanArgs = Vec<LeanValue>;

/// Represents the return value of a Lean ABI function.
pub type LeanResult = Result<LeanValue, String>;
