use crate::muse::muse_enum::MuseName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Muse {
    pub name: MuseName,
    pub description: String,
    pub pdl_path: String,
}
