use ragit_index_types::index_struct::Index;
use ragit_types::ApiError;
use ragit_model_query_response::ModelQueryResponse;
use ragit_types::query_turn::QueryTurn;

pub mod prelude;

use ragit_memory_monitor::MemoryMonitor;

pub async fn query(
    index: &Index,
    query_str: &str,
    turns: Vec<QueryTurn>,
    model_override: Option<String>,
    memory_monitor: &mut MemoryMonitor,
) -> Result<ModelQueryResponse, ApiError> {
    memory_monitor.verbose(&format!("ragit_index_query::query: Executing query: '{}'", query_str));
    memory_monitor.verbose(&format!("ragit_index_query::query: Number of chunks in index: {}", index.chunks.len()));

    let mut results = Vec::new();
    for chunk in index.get_chunks() {
        if chunk.data.as_str().contains(query_str) {
            results.push(format!("Found in chunk from file {:?}: {}", chunk.file, chunk.data));
        }
    }

    let response_message = if results.is_empty() {
        "No relevant information found.".to_string()
    } else {
        results.join("\n")
    };

    memory_monitor.verbose(&format!("ragit_index_query::query: Query response generated. Length: {}", response_message.len()));
    Ok(ModelQueryResponse::new(response_message))
}
