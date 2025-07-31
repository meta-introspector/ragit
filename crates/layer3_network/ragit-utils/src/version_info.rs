use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VersionInfo {
    pub version: String,
    pub compatible: bool,
}
