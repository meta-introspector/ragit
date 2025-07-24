pub use crate::prelude::*;
use ragit_index_types::Index;
use ragit_types::chunk::chunk_struct::Chunk;
use ragit_api::Request;
use serde_json::Value;
use anyhow::Error;
pub use ragit_index_chunk_methods::retrieve_chunks::retrieve_chunks;
pub use ragit_prompt_management::get_prompt::get_prompt;
pub use ragit_model::Model;
pub use ragit_api::Schema;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueryResponse {
    pub multi_turn_schema: Option<MultiTurnSchema>,
    pub retrieved_chunks: Vec<Chunk>,
    pub response: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueryTurn {
    pub query: String,
    pub response: QueryResponse,
}

impl QueryTurn {
    pub fn new(query: String, response: QueryResponse) -> Self {
        QueryTurn { query, response }
    }
}

pub async fn single_turn(
    index: &Index,
    q: &str,
) -> Result<String, Error> {
    let result = query(index, q, vec![], None).await?;
    Ok(result.response)
}

pub async fn query(
    index: &Index,
    q: &str,
    history: Vec<QueryTurn>,
    schema: Option<Schema>,
) -> Result<QueryResponse, Error> {
    let (multi_turn_schema, rephrased_query) = if history.is_empty() || !index.query_config.enable_rag || index.chunk_count == 0 {
        (None, q.to_string())
    } else {
        let multi_turn_schema = rephrase_multi_turn(
            index,
            select_turns_for_context(&history, q),
        ).await?;
        let rephrased_query = if multi_turn_schema.is_query && multi_turn_schema.in_context {
            multi_turn_schema.query.clone()
        } else {
            q.to_string()
        };

        (Some(multi_turn_schema), rephrased_query)
    };
    let chunks = retrieve_chunks(index, &rephrased_query, index.query_config.max_retrieval).await?;

    let response = if chunks.is_empty() {
        raw_request(
            index,
            q,
            history.iter().flat_map(|h| vec![h.query.clone(), h.response.response.clone()]).collect(),
            schema,
        ).await?
    } else {
        answer_query_with_chunks(
            index,
            &rephrased_query,
            chunks.clone(),
            schema,
        ).await?
    };

    Ok(QueryResponse {
        multi_turn_schema,
        retrieved_chunks: chunks,
        response,
    })
}

pub async fn answer_query_with_chunks(
    index: &Index,
    query: &str,
    chunks: Vec<Chunk>,
    schema: Option<Schema>,
) -> Result<String, Error> {
    let chunks = ragit_index_chunk_methods::get_merged_chunk_of_file::merge_and_convert_chunks(index, chunks)?;
    let mut tera_context = tera::Context::new();
    tera_context.insert(
        "chunks",
        &chunks,
    );
    tera_context.insert(
        "query",
        &query,
    );

    let Pdl { messages, .. } = ragit_pdl::parse_pdl(
        &get_prompt(index, "answer_query")?,
        &tera_context,
        "/",  // TODO: `<|media|>` is not supported for this prompt
        true,
    )?;

    let request = Request {
        messages,
        schema: schema.clone(),
        timeout: index.api_config.timeout,
        max_retry: index.api_config.max_retry,
        sleep_between_retries: index.api_config.sleep_between_retries,
        dump_pdl_at: index.api_config.create_pdl_path(&index.root_dir, "answer_query_with_chunks").map(|p| p.to_str().unwrap().to_string()),
        dump_json_at: index.api_config.dump_log_at(&index.root_dir).map(|p| p.to_str().unwrap().to_string()),
        model: index.api_config.get_model_by_name(&index.api_config.model.clone().into())?, // Corrected method call
        dump_api_usage_at: index.api_config.dump_api_usage_at(&index.root_dir, "answer_query_with_chunks"),
        schema_max_try: 3,
        ..Request::default()
    };

    let response = match schema {
        Some(schema) => {
            let result = request.send_and_validate::<Value>(Value::Null).await?;
            ragit_pdl::render_pdl_schema(&schema, &result)?
        },
        None => request.send().await?.get_message(0).unwrap().to_string(),
    };

    Ok(response)
}

pub async fn rephrase_multi_turn(
    index: &Index,
    turns: Vec<String>,
) -> Result<MultiTurnSchema, Error> {
    let turns_json = Value::Array(turns.iter().map(|turn| Value::String(turn.to_string())).collect());
    let turns_json = serde_json::to_string_pretty(&turns_json)?;
    let mut tera_context = tera::Context::new();
    tera_context.insert("turns", &turns_json);

    let Pdl { messages, schema } = ragit_pdl::parse_pdl(
        &get_prompt(index, "multi_turn")?,
        &tera_context,
        "/",  // TODO: `<|media|>` is not supported for this prompt
        true,
    )?;

    let request = Request {
        messages,
        frequency_penalty: None,
        max_tokens: None, // Removed max_output_tokens
        temperature: None,
        timeout: index.api_config.timeout,
        max_retry: index.api_config.max_retry,
        sleep_between_retries: index.api_config.sleep_between_retries,
        dump_pdl_at: index.api_config.create_pdl_path(&index.root_dir, "rephrase_multi_turn").map(|p| p.to_str().unwrap().to_string()),
        dump_json_at: index.api_config.dump_log_at(&index.root_dir).map(|p| p.to_str().unwrap().to_string()),
        model: index.api_config.get_model_by_name(&index.api_config.model.clone().into())?, // Corrected method call
        dump_api_usage_at: index.api_config.dump_api_usage_at(&index.root_dir, "rephrase_multi_turn"),
        schema,
        schema_max_try: 3,
    };

    Ok(request.send_and_validate::<MultiTurnSchema>(MultiTurnSchema::default()).await?)
}

pub async fn raw_request(
    index: &Index,
    query: &str,
    history: Vec<String>,
    schema: Option<Schema>,
) -> Result<String, Error> {
    let mut tera_context = tera::Context::new();
    tera_context.insert("query", &query);
    tera_context.insert("history", &history);

    let Pdl { messages, .. } = ragit_pdl::parse_pdl(
        &get_prompt(index, "raw")?,
        &tera_context,
        "/",  // TODO: `<|media|>` is not supported for this prompt
        true,
    )?;
    let request = Request {
        messages,
        schema: schema.clone(),
        timeout: index.api_config.timeout,
        max_retry: index.api_config.max_retry,
        sleep_between_retries: index.api_config.sleep_between_retries,
        dump_pdl_at: index.api_config.create_pdl_path(&index.root_dir, "raw_request").map(|p| p.to_string_lossy().into_owned()),
        dump_json_at: index.api_config.dump_log_at(&index.root_dir).map(|p| p.to_str().unwrap().to_string()),
        model: index.api_config.get_model_by_name(&index.api_config.model.clone().into())?, // Corrected method call
        dump_api_usage_at: index.api_config.dump_api_usage_at(&index.root_dir, "raw_request"),
        schema_max_try: 3,
        ..Request::default()
    };

    let response = match schema {
        Some(schema) => {
            let result = request.send_and_validate::<Value>(Value::Null).await?;
            ragit_pdl::render_pdl_schema(&schema, &result)?
        },
        None => request.send().await?.get_message(0).unwrap().to_string(),
    };

    Ok(response)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MultiTurnSchema {
    pub is_query: bool,
    pub in_context: bool,
    pub query: String,
}

impl Default for MultiTurnSchema {
    fn default() -> Self {
        MultiTurnSchema {
            is_query: false,
            in_context: false,
            query: String::new(),
        }
    }
}

fn select_turns_for_context(history: &[QueryTurn], query: &str) -> Vec<String> {
    match history.len() {
        0 => unreachable!(),
        1 => vec![
            history[0].query.to_string(),
            history[0].response.response.to_string(),
            query.to_string(),
        ],
        _ => {
            let last_turn = history.last().unwrap();

            match &last_turn.response.multi_turn_schema {
                None => vec![
                    last_turn.query.to_string(),
                    last_turn.response.response.to_string(),
                    query.to_string(),
                ],
                // use rephrased query if in-context
                Some(MultiTurnSchema {
                    is_query: true,
                    in_context: true,
                    query: rephrased_query,
                }) => vec![
                    rephrased_query.to_string(),
                    last_turn.response.response.to_string(),
                    query.to_string(),
                ],
                // still in context, but is not a query (e.g. greetings)
                Some(MultiTurnSchema {
                    is_query: false,
                    in_context: true,
                    query: _,
                }) => {
                    let before_last_turn = history.get(history.len() - 2).unwrap();

                    vec![
                        before_last_turn.query.to_string(),
                        before_last_turn.response.response.to_string(),
                        last_turn.query.to_string(),
                        last_turn.response.response.to_string(),
                        query.to_string(),
                    ]
                },
                // start a new context
                Some(MultiTurnSchema {
                    in_context: false,
                    ..
                }) => vec![
                    last_turn.query.to_string(),
                    last_turn.response.response.to_string(),
                    query.to_string(),
                ],
            }
        },
    }
}
