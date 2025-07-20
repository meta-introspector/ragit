use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct VersionInfo {
    pub version: String,
    pub compatible: bool,
}

pub fn get_compatibility_warning(version_info: &VersionInfo) -> Option<String> {
    None
}