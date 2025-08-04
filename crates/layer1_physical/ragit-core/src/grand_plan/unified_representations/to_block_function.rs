/// Conceptually transforms a struct into its block representation.
pub fn to_block<T: std::fmt::Debug>(s: &T) -> String {
    format!("Block representation of {:?}", s)
}
