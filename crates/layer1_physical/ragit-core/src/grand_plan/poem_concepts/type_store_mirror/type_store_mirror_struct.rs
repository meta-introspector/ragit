use crate::grand_plan::unified_stores::type_store_enum::TypeStore;

#[derive(Debug, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// By 17's measure, mirrors gleam,
/// Reflecting every stored TypeStore stream.
pub struct TypeStoreMirror(pub TypeStore);
