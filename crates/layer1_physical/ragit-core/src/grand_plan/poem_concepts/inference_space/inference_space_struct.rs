use ragit_macros::OurMacro;

#[derive(Debug, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// In Inference Space, a rule takes flight, (λ)
/// To weave the darkness into light.
pub struct InferenceSpace<F, T> where F: Fn(usize) -> T {
    pub lambda: F,
}
