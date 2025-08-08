use serde::{Deserialize, Serialize};
use ragit_types::query::QueryConfig;

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
