/// Represents the different "flavors" of guidance that can be applied.
#[derive(Debug, Clone)]
pub enum Prompt {
    Poem,
    Emoji,
    BottPeriodicity,
    Discussion(String), // The string will hold the content of the .pdl file
}