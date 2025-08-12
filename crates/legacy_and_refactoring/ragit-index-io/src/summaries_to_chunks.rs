
use ragit_index_types::index_struct::Index;
use ragit_types::ApiError;
use ragit_types::chunk::chunk_struct::Chunk;
use ragit_pdl::{Pdl, parse_pdl};
use ragit_api::request::Request;
use serde_json::json;
use serde_json::Value;
use ragit_prompt_management::get_prompt::get_prompt;
pub async fn summaries_to_chunks(
    index: &Index,
    query: &str,
    chunks: Vec<Chunk>,
) -> Result<Vec<Chunk>, ApiError> {
    let Pdl { messages, schema } = parse_pdl(
        &get_prompt(&index.prompts, &index.root_dir, "summarize_chunks")?,
        &tera::Context::from_value(json!({
            "query": query,
            "chunks": chunks.iter().map(|c| c.summary.clone()).collect::<Vec<_>>(),
        }))?,
        ".",
        true,
    )?;

    let request = Request {
        messages,
        model: ragit_api::get_model_by_name(&index.models, &index.api_config.model.clone())?,
        temperature: Some(0.0),
        // max_tokens: Some(index.query_config.max_output_tokens),
        schema: schema.clone(),
        ..Request::default()
    };

    let result = request.send_and_validate::<Value>(Value::Null).await?;

    Ok(vec![])
}
