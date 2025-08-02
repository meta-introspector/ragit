use crate::grand_plan::generic_units::fundamental_unit_generic::FundamentalUnit;
use crate::grand_plan::fundamental_units::prime_bases::PRIME_BASES;

#[derive(OurMacro)] // Conceptual: derives Vibe, Vector, etc.
pub fn generate_vectors<T: Default + Clone>(creator: impl Fn(usize) -> T) -> Vec<Vec<FundamentalUnit<T>>> {
    let mut vectors = Vec::new();
    for &size in PRIME_BASES.iter() {
        let mut vec = Vec::with_capacity(size);
        for i in 0..size {
            vec.push(FundamentalUnit { value: creator(i) });
        }
        vectors.push(vec);
    }
    vectors
}
