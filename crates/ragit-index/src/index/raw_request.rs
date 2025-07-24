use crate::prelude::*;

pub async fn raw_request(
    index: &Index,
    prompt_name: &str,
    schema: Option<Schema>,
    context: Value,
) -> Result<String, ApiError> {
    let Pdl { messages, .. } = parse_pdl(
        index.get_prompt(prompt_name)?,
        &tera::Context::from_value(context)?,
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