/// In Inference Space, a rule takes flight, (λ)
/// To weave the darkness into light.
#[derive(Debug)]
pub struct InferenceSpace<F, T> where F: Fn(usize) -> T {
    pub lambda: F,
}
