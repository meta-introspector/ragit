/// Conceptually transforms a struct into its Gödel number representation.
pub fn to_goedel_number<T: std::fmt::Debug>(s: &T) -> String {
    format!("Gödel number of {:?}", s)
}
