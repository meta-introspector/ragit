use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum LoadMode {
    Normal,
    QuickCheck,
}
