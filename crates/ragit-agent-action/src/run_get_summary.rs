use crate::action_result_enum::ActionResult;
use ragit_index_types::index_struct::Index;
use ragit_types::ApiError;

pub(crate) async fn run_get_summary(
    argument: &str,
    index: &Index,
) -> Result<ActionResult, ApiError> {
    // The summary must exist. Otherwise, this action should have been filtered out.
    let summary = index.get_summary().unwrap();
    Ok(ActionResult::GetSummary(summary.to_string()))
}
