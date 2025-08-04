use serde::{Deserialize, Serialize};

use ragit_macros::OurMacro;

#[derive(Debug, Clone, Serialize, Deserialize, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// Represents a generic value that can be passed across the TypeScript ABI.
pub enum TypeScriptValue {
    U32(u32),
    String(String),
    Bool(bool),
    // Add more types as needed
}

/// Represents the arguments passed to a TypeScript ABI function.
pub type TypeScriptArgs = Vec<TypeScriptValue>;

/// Represents the return value of a TypeScript ABI function.
pub type TypeScriptResult = Result<TypeScriptValue, String>;
