use crate::action_result_enum::ActionResult;
use ragit_index_types::index_struct::Index;
use ragit_model::Model;
use ragit_types::ApiError;

pub(crate) async fn run_simple_rag(
    argument: &str,
    _index: &Index,
) -> Result<ActionResult, ApiError> {
    let model = Model::dummy();
    let response = model.query(argument).await?;
    Ok(ActionResult::SimpleRag(response))
}
