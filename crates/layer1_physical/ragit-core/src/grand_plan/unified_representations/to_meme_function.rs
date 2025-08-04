/// Conceptually transforms a struct into its meme representation.
pub fn to_meme<T: std::fmt::Debug>(s: &T) -> String {
    format!("Meme representation of {:?}", s)
}
