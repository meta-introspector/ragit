use crate::action_result_enum::ActionResult;
use ragit_index_types::index_struct::Index;
use ragit_model::Model;
use ragit_types::ApiError;

pub(crate) async fn run_simple_rag(
    _argument: &str,
    _index: &Index,
) -> Result<ActionResult, ApiError> {
    let model = Model::new();
    let response = model.query("What is the meaning of life?").await?;
    Ok(ActionResult::SimpleRag(response))
}
