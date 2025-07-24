use crate::index_struct::Index;
use ragit_error::ApiError;
use ragit_types::chunk::chunk_struct::Chunk;
use ragit_api::{Pdl, parse_pdl};
use ragit_api::request::Request;
use serde_json::json;
use tera::Context;
use serde_json::Value;

pub async fn summaries_to_chunks(
    index: &Index,
    query: &str,
    chunks: Vec<Chunk>,
) -> Result<Vec<Chunk>, ApiError> {
    let Pdl { messages, schema } = parse_pdl(
        index.get_prompt("summarize_chunks")?,
        &tera::Context::from_value(json!({
            "query": query,
            "chunks": chunks.iter().map(|c| c.summary.clone()).collect::<Vec<_>>(),
        }))?,
        ".",
        true,
    )?;

    let request = Request {
        messages,
        model: index.api_config.model.clone(),
        temperature: Some(0.0),
        max_tokens: Some(index.query_config.max_output_tokens),
        schema: Some(schema.clone()),
        ..Request::default()
    };

    let result = request.send_and_validate::<Value>(Value::Null).await?;

    Ok(vec![])
}
