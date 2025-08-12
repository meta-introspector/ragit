use serde::Serialize;
use super::action_enum::Action;
use super::action_result_enum::ActionResult;

#[derive(Serialize)]
pub struct ActionTrace {
    pub action: Action,
    pub argument: Option<String>,
    pub result: ActionResult,
}
