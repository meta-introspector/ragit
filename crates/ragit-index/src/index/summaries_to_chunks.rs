use crate::prelude::*;

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
        model: index.get_model_by_name(&index.api_config.model)?,
        temperature: Some(0.0),
        max_tokens: Some(index.query_config.max_output_tokens),
        schema: Some(schema.clone()),
        ..Request::default()
    };

    let result = request.send_and_validate::<Value>(Value::Null).await?;

    Ok(vec![])
}