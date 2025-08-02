use crate::grand_plan::fundamental_unit::FundamentalUnit;

pub const PRIME_BASES: [usize; 7] = [2, 3, 5, 7, 11, 17, 19];

pub fn generate_char_vectors() -> Vec<Vec<FundamentalUnit>> {
    let mut vectors = Vec::new();
    for &size in PRIME_BASES.iter() {
        let mut vec = Vec::with_capacity(size);
        for i in 0..size {
            vec.push(FundamentalUnit::Char((i as u8 + b'a') as char));
        }
        vectors.push(vec);
    }
    vectors
}
