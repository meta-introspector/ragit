use crate::action_result_enum::ActionResult;

impl ActionResult {
    pub fn render_read_file_short(rendered: &RenderedChunk) -> String {
        rendered.human_data.clone()
    }
}
