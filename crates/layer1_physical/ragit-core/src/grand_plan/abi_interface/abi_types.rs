use serde::{Deserialize, Serialize};

/// Represents a generic value that can be passed across the ABI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AbiValue {
    U32(u32),
    String(String),
    Bool(bool),
    // Add more types as needed
}

/// Represents the arguments passed to an ABI function.
pub type AbiArgs = Vec<AbiValue>;

/// Represents the return value of an ABI function.
pub type AbiResult = Result<AbiValue, String>;
