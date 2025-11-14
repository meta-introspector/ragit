// use crate::error::Error;
// use ragit_model_query_response::ModelQAResult;
// use serde_json;

pub fn log_qa_results(_results: &[()]) -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!("FIX ME LATER: Fix the bootstrap first and this code later."))
    // let log_entry = serde_json::to_string_pretty(results)?;
    // let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
    // std::fs::write(format!("qa_results_{}.json", timestamp), log_entry)?;
    // Ok(())
}
