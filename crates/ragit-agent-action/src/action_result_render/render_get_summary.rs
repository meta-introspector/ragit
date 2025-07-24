use crate::action_result_enum::ActionResult;

impl ActionResult {
    pub fn render_get_summary(summary: &String) -> String {
        summary.clone()
    }
}
