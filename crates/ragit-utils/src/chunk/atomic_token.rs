use serde::{Deserialize, Serialize};
use crate::chunk::chunk_extra_info::ChunkExtraInfo;
use ragit_pdl::Image;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum AtomicToken {
    String { data: String, char_len: usize },
    Image(Image),
    WebImage { subst: String, url: String },
    PageBreak,
    ChunkExtraInfo(ChunkExtraInfo),
}

pub fn escape_pdl_tokens(s: &str) -> String {
    s.to_string()
}
