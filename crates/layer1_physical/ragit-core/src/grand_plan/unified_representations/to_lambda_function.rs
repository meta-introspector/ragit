/// Conceptually transforms a struct into its lambda representation.
pub fn to_lambda<T: std::fmt::Debug>(s: &T) -> String {
    format!("Lambda representation of {:?}", s)
}
