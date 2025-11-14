use crate::action_result_enum::ActionResult;use ragit_types::QueryResponse;impl ActionResult {    pub fn render_simple_rag(_response: &QueryResponse) -> String {        panic!("FIX ME LATER: Fix the bootstrap first and this code later.");
        // format!("{} {}", constants::RENDER_SIMPLE_RAG, response.response.clone())
    }}