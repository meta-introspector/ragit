use ragit_index_types::index_struct::Index;
use ragit_types::ApiError;
use ragit_model_query_response::ModelQueryResponse;
use ragit_types::query_turn::QueryTurn;
use ragit_tfidf::{tokenize, TfidfState};
use ragit_utils::query::Keywords;

pub mod prelude;

use ragit_memory_monitor::MemoryMonitor;

pub async fn query(
    index: &Index,
    query_str: &str,
    _turns: Vec<QueryTurn>,
    _model_override: Option<String>,
    memory_monitor: &mut MemoryMonitor,
) -> Result<ModelQueryResponse, ApiError> {
    memory_monitor.verbose(&format!("ragit_index_query::query: Executing TF-IDF query: '{}'", query_str));
    memory_monitor.verbose(&format!("ragit_index_query::query: Number of chunks in index: {}", index.chunks.len()));

    let keywords = Keywords(tokenize(query_str));
    memory_monitor.verbose(&format!("ragit_index_query::query: Tokenized query into keywords: {:?}", keywords.0));

    let mut tfidf_state = TfidfState::new(&keywords);

    memory_monitor.verbose("ragit_index_query::query: Building TF-IDF state from index chunks...");
    for chunk in index.get_chunks() {
        let tokens = tokenize(chunk.data.as_str());
        tfidf_state.add_doc(chunk.uid.clone(), tokens);
    }
    memory_monitor.verbose(&format!("ragit_index_query::query: TF-IDF state built with {} documents.", index.chunks.len()));

    memory_monitor.verbose("ragit_index_query::query: Performing TF-IDF search...");
    let search_results = tfidf_state.search(&keywords);
    memory_monitor.verbose(&format!("ragit_index_query::query: Found {} results.", search_results.len()));


    let mut results = Vec::new();
    for result in search_results.iter().take(10) { // Limit to top 10 results
        if let Some(chunk) = index.get_chunks().iter().find(|c| c.uid == result.doc_id) {
            results.push(format!("Score: {:.4} - Found in chunk from file {:?}:\n---\n{}\n---\n", result.score, chunk.file, chunk.data));
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
