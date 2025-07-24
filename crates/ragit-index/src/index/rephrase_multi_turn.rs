use crate::prelude::*;

pub async fn rephrase_multi_turn(
    index: &Index,
    turns: Vec<String>,
) -> Result<MultiTurnSchema, ApiError> {
    let turns_json = Value::Array(
        turns
            .into_iter()
            .map(|turn| Value::String(turn.to_string()))
            .collect(),
    );

    let Pdl { messages, schema } = parse_pdl(
        index.get_prompt("multi_turn")?,
        &tera::Context::from_value(json!({
            "turns": turns_json,
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

    request
        .send_and_validate::<MultiTurnSchema>(MultiTurnSchema::default())
        .await
}