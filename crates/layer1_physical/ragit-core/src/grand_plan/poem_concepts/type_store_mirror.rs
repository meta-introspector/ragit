use crate::grand_plan::unified_store::TypeStore;

/// By 17's measure, mirrors gleam,
/// Reflecting every stored TypeStore stream.
#[derive(Debug)]
pub struct TypeStoreMirror(pub TypeStore);
