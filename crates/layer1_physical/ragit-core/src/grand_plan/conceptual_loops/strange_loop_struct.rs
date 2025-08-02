use serde::{Deserialize, Serialize};

use ragit_macros::OurMacro;

#[derive(Debug, Clone, Serialize, Deserialize, OurMacro)]
pub struct StrangeLoop {
    pub hott_unimath_path: String,
    pub digital_fungus_mycellium: String,
    pub zkp_circuit: String,
    pub goedel_number: String,
}
