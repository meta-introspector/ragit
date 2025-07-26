use crate::action_result_enum::ActionResult;
use crate::constants;

impl ActionResult {
    pub fn render_read_chunk_too_many(query: &str, chunk_uids: usize) -> String {
        // `Action::ReadChunk`'s default abbrev is 9.
        if query.len() >= 9 {
            format!(constants::RENDER_CHUNK_TOO_MANY_UNLUCKY, chunk_uids, query)
        }

        else {
            format!("{} {} {} {}", constants::RENDER_CHUNK_TOO_MANY_AMBIGUOUS, query, chunk_uids, query)
        }
    }
}
