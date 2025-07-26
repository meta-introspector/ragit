use crate::action_result_enum::ActionResult;
use crate::constants;
use ragit_types::Uid;

impl ActionResult {
    pub fn render_no_such_chunk(query: &str) -> String {
        if !Uid::is_valid_prefix(&query) {
            format!(constants::RENDER_NO_SUCH_CHUNK_INVALID_UID, query)
        }

        else {
            format!("{} {}", constants::RENDER_NO_SUCH_CHUNK, query)
        }
    }
}
