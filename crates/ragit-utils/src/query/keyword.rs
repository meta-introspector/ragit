use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Keywords;

impl Keywords {
    pub fn tokenize(&self) -> HashMap<String, f32> {
        HashMap::new()
    }
}