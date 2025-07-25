use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct IIStatus {
    // Placeholder for inverted index status
    pub enabled: bool,
    pub last_updated: Option<String>,
}
