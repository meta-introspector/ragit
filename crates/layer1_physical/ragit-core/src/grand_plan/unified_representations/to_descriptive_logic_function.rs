/// Conceptually transforms a struct into its descriptive logic representation.
pub fn to_descriptive_logic<T: std::fmt::Debug>(s: &T) -> String {
    format!("Descriptive logic of {:?}", s)
}
