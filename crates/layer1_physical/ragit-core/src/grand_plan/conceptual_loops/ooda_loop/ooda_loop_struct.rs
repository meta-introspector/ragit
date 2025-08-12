use serde::{Deserialize, Serialize};

use ragit_macros::OurMacro;

#[derive(Debug, Clone, Serialize, Deserialize, OurMacro)]
pub struct OodaLoop {
    pub observe: String,
    pub orient: String,
    pub decide: String,
    pub act: String,
}
