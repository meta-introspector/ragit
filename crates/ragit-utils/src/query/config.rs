use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct QueryConfig {
    pub enable_ii: bool,
}

impl Default for QueryConfig {
    fn default() -> Self {
        Self {
            enable_ii: true,
        }
    }
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PartialQueryConfig {
    pub enable_ii: Option<bool>,
}

impl PartialQueryConfig {
    pub fn apply_to(&self, config: &mut QueryConfig) {
        if let Some(enable_ii) = self.enable_ii {
            config.enable_ii = enable_ii;
        }
    }
}