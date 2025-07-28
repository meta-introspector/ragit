use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Keywords(pub Vec<String>);

impl From<Vec<String>> for Keywords {
    fn from(keywords: Vec<String>) -> Self {
        Keywords(keywords)
    }
}

impl Keywords {
    pub fn new() -> Self {
        Keywords(Vec::new())
    }

    pub fn tokenize(&self) -> HashMap<String, f32> {
        let mut map = HashMap::new();
        for keyword in &self.0 {
            map.insert(keyword.clone(), 1.0); // Assign a default weight of 1.0 for now
        }
        map
    }
}
