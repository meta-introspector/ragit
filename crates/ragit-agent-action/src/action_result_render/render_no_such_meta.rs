use crate::action_result_enum::ActionResult;
use crate::constants;

impl ActionResult {
    pub fn render_no_such_meta(key: &str, similar_keys: &Vec<String>) -> String {
        format!(constants::RENDER_NO_SUCH_META, key, if !similar_keys.is_empty() { format!(constants::RENDER_NO_SUCH_META_SIMILAR, similar_keys) } else { String::new() })
    }
}
