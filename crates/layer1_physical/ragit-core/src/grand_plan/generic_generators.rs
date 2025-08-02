use crate::grand_plan::generic_unit::FundamentalUnit;

pub const PRIME_BASES: [usize; 7] = [2, 3, 5, 7, 11, 17, 19];

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
