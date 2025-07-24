use crate::action_result_enum::ActionResult;
use crate::constants;

impl ActionResult {
    pub fn render_no_such_dir(dir: &str, similar_dirs: &Vec<String>) -> String {
        format!(
            constants::RENDER_NO_SUCH_DIR,
            dir,
            if !similar_dirs.is_empty() {
                format!(constants::RENDER_NO_SUCH_DIR_SIMILAR, similar_dirs.join("\n"))
            } else {
                String::new()
            },
        )
    }
}
