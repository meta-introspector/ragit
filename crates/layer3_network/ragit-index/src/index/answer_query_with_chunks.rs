use crate::prelude::*;

pub async fn answer_query_with_chunks(
    index: &Index,
    query: &str,
    schema: Option<Schema>,
    chunks: Vec<Chunk>,
) -> Result<String, ApiError> {
    let rendered_chunks: Vec<RenderedChunk> = chunks
        .iter()
        .map(|c| c.render(index))
        .collect::<Result<Vec<RenderedChunk>, ApiError>>()?;

    let chunks = merge_and_convert_chunks(index, rendered_chunks)?;

    let Pdl { messages, .. } = parse_pdl(
        index.get_prompt("answer_query")?,
        &tera::Context::from_value(json!({
            "query": query,
            "chunks": chunks,
        }))?,
        ".",
        true,
    )?;

    let request = Request {
        messages,
        model: index.get_model_by_name(&index.api_config.model)?,
        temperature: Some(0.0),
        max_tokens: Some(index.query_config.max_output_tokens),
        schema,
        ..Request::default()
    };

    let result = request.send_and_validate::<Value>(Value::Null).await?;

    if let Some(schema) = schema {
        render_pdl_schema(&schema, &result)?
    } else {
        result.to_string()
    }
}

fn merge_and_convert_chunks(
    index: &Index,
    chunks: Vec<RenderedChunk>,
) -> Result<Vec<String>, ApiError> {
    let mut result = vec![];
    let mut current_chunk = String::new();
    let mut current_tokens = 0;

    for chunk in chunks {
        let chunk_tokens = count_tokens(&chunk.human_data);
        if current_tokens + chunk_tokens > index.query_config.max_input_tokens {
            result.push(current_chunk);
            current_chunk = String::new();
            current_tokens = 0;
        }
        current_chunk.push_str(&chunk.human_data);
        current_tokens += chunk_tokens;
    }

    if !current_chunk.is_empty() {
        result.push(current_chunk);
    }

    Ok(result)
}