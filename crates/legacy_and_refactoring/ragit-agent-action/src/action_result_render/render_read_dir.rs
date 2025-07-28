use crate::action_result_enum::ActionResult;
use ragit_agent::file_tree::FileTree;

impl ActionResult {
    pub fn render_read_dir(file_tree: &FileTree) -> String {
        file_tree.render()
    }
}
