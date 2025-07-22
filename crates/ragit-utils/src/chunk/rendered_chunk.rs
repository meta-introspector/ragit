use serde::{Deserialize, Serialize};
use crate::Uid;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum MultiModalContent {
    Text { content: String },
    Image { uid: Uid },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RenderedChunk {
    pub pdl_data: String,
    pub human_data: String,
    pub raw_data: Vec<MultiModalContent>,
    pub source: String,
}
