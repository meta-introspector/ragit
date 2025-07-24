use crate::action_result_enum::ActionResult;
use ragit_types::Chunk;

impl ActionResult {
    pub fn render_read_chunk(chunk: &Chunk) -> String {
        chunk.data.clone()
    }
}
