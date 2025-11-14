use crate::index::index_struct::Index;
use crate::error::Error;
use crate::Path;

// Define AddMode enum
#[derive(Debug, PartialEq, Clone, Copy)] // Add necessary derives
pub enum AddMode {
    Reject,
    // Add other variants as needed based on future errors or context
}

impl AddMode {
    pub fn parse_flag(flag: &str) -> Option<Self> {
        match flag {
            "reject" => Some(AddMode::Reject),
            _ => None,
        }
    }
}

// Define AddResult struct
#[derive(Debug, PartialEq, Clone)] // Add necessary derives
pub struct AddResult {
    // Add fields as needed based on future errors or context
    pub success: bool,
}

impl Index {
    pub fn add(&mut self, file: Path) -> Result<(), Error> {
        self.staged_files.push(file);
        Ok(())
    }
}