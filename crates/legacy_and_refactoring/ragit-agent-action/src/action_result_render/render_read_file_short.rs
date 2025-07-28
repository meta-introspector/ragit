use crate::action_result_enum::ActionResult;
use ragit_types::chunk::rendered_chunk::RenderedChunk;

impl ActionResult {
    pub fn render_read_file_short(rendered: &RenderedChunk) -> String {
        rendered.human_data.clone()
    }
}
