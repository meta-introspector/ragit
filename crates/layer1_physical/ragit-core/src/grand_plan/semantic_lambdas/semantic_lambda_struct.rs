#[derive(Debug, Clone, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// Represents a step in the 8-fold creative path, linking semantics to a computational form.
pub struct SemanticLambda {
    /// The emoji representing the concept.
    pub emoji: char,
    /// The number (0-7) in the 8-fold path.
    pub number: u32,
    /// The poetic name of the step.
    pub name: &'static str,
    /// The philosophical meaning of the step.
    pub meaning: &'static str,
    /// A string representation of the Rust lambda that executes the step's logic.
    pub lambda_str: &'static str,
}
