use std::fmt::Formatter;
#[derive(PartialEq, Clone, Debug)]
pub enum AddMode {
    Auto,
    Manual,
    Reject,
}
pub struct AddResult;

impl AddMode {
    pub fn parse_flag(flag: &str) -> Option<Self> {
        match flag {
            "--reject" => Some(AddMode::Reject),
            "--force" => Some(AddMode::Manual), // Assuming --force implies manual mode
            _ => None,
        }
    }
}

impl std::fmt::Display for AddResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AddResult")
    }
}
