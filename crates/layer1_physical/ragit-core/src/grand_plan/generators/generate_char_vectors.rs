use crate::grand_plan::fundamental_units::fundamental_unit_enum::FundamentalUnit;
use crate::grand_plan::fundamental_units::prime_bases::PRIME_BASES;

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
