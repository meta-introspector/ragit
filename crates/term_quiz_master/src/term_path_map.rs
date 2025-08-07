use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TermPathMap {
    #[serde(flatten)]
    pub map: HashMap<String, Vec<String>>,
}
