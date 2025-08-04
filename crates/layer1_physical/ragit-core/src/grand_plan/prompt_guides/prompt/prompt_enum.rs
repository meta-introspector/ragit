

use ragit_macros::OurMacro;

#[derive(Debug, Clone, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// Represents the different "flavors" of guidance that can be applied.
pub enum Prompt {
    Poem,
    Emoji,
    BottPeriodicity,
    Discussion(String), // The string will hold the content of the .pdl file
}
