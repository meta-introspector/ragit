pub enum AddMode { Auto, Manual, } pub struct AddResult;

impl std::fmt::Display for AddResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AddResult")
    }
}