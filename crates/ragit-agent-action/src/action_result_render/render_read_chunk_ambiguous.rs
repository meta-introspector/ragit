use crate::action_result_enum::ActionResult;
use crate::constants;
use ragit_types::{Uid, Chunk};

impl ActionResult {
    pub fn render_read_chunk_ambiguous(query: &str, chunks: &Vec<Chunk>) -> String {
        let chunks = chunks.iter().enumerate().map(
            |(index, chunk)| format!(
                constants::RENDER_CHUNK_AMBIGUOUS_CHUNK,
                index + 1,
                chunk.render_source(),
                chunk.uid.abbrev(query.len() + 4),
                chunk.title,
                chunk.summary,
            )
        ).collect::<Vec<_>>().join("\n\n");
        format!(constants::RENDER_CHUNK_AMBIGUOUS, query, chunks)
    }
}
