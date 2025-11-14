use crate::chunk::chunk_extra_info::ChunkExtraInfo;
use crate::image::Image;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AtomicToken {
    String { data: String, char_len: usize },
    Image(Image),
    WebImage { subst: String, url: String },
    PageBreak,
    ChunkExtraInfo(ChunkExtraInfo),
}

impl AtomicToken {
    pub fn len(&self, image_size: usize) -> usize {
        match self {
            AtomicToken::String { char_len, .. } => *char_len,
            AtomicToken::Image(_) | AtomicToken::WebImage { .. } => image_size,
            AtomicToken::PageBreak | AtomicToken::ChunkExtraInfo(_) => 0,
        }
    }
}

pub fn escape_pdl_tokens(s: &str) -> String {
    s.to_string()
}
