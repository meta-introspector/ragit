use crate::action_result_enum::ActionResult;
use ragit_index_types::index_struct::Index;
use ragit_types::ApiError;
// use ragit_index_query::query;
// use ragit_memory_monitor::MemoryMonitor;

pub(crate) async fn run_simple_rag(
    _argument: &str,
    _index: &Index,
    _memory_monitor: &mut MemoryMonitor,
) -> Result<ActionResult, ApiError> {
    panic!("FIX ME LATER: Fix the bootstrap first and this code later.");
    // let response = query(index, argument, vec![], None, memory_monitor).await?;
    // Ok(ActionResult::SimpleRag(response))
}