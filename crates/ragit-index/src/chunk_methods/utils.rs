use crate::prelude::*;

pub fn merge_and_convert_chunks(
    _index: &crate::index::index_struct::Index,
    _chunks: Vec<RenderedChunk>,
) -> Result<Vec<RenderedChunk>> {
    Err(anyhow!("Not implemented"))
}

pub fn merge_overlapping_strings(_s1: &[u8], _s2: &[u8]) -> String {
    String::new()
}

pub fn into_multi_modal_contents(data: &str, _images: &[Uid]) -> Vec<MultiModalContent> {
    // Placeholder implementation
    vec![MultiModalContent::Text {
        content: data.to_string(),
    }]
}
