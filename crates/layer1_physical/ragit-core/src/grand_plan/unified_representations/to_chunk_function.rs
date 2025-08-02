/// Conceptually transforms a struct into its chunk representation.
pub fn to_chunk<T: std::fmt::Debug>(s: &T) -> String {
    format!("Chunk representation of {:?}", s)
}
