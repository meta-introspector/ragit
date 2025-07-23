use serde::{Deserialize, Serialize};
use crate::Uid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageSchema {
    pub uid: Uid,
    pub extracted_text: String,
    pub explanation: String,
    pub size: u64,
    pub bytes: Vec<u8>,
}
