#[derive(Debug, Clone, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
pub struct FundamentalUnit<T> {
    pub value: T,
}