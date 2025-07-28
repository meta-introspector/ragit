use ragit_error_conversions::WrappedPdlError;
use ragit_types::build_config::BuildConfig;
use ragit_types::chunk::atomic_token::AtomicToken;
use ragit_types::api_config::ApiConfig;
use ragit_types::ApiError as Error;
//use regex::Regex;
//use serde_json::Value as JsonValue;
use sha3::{Digest, Sha3_256};
use ragit_types::pdl_types::{
    //Message,
    MessageContent, Role};
use ragit_fs::{normalize};
use ragit_types::chunk::chunk_struct::Chunk;
use ragit_types::chunk::chunk_source::ChunkSource;
use ragit_types::uid::uid_from_hash;

//use ragit_api::Request;
use ragit_model::Model;
//use ragit_pdl::{Pdl, parse_pdl};
use std::path::PathBuf;
//use std::time::Instant;
use crate::chunk_creation_utils;
use ragit_pdl::Pdl;
pub async fn create_chunk_from(
    tokens: &[AtomicToken],
    config: &BuildConfig,
    file: String,
    file_index: usize,
    api_config: &ApiConfig,
    pdl: &str,
    build_info: ragit_types::chunk::chunk_struct::ChunkBuildInfo,
    dry_run_llm: bool,
) -> Result<Chunk, Error> {
    let mut dummy_context = tera::Context::new();
    dummy_context.insert("chunk", "placeholder");

    let Pdl { messages: mut prompt, .. } = ragit_pdl::parse_pdl(
        pdl,
        &dummy_context,
        &file, // Assuming 'file' can be used as the current directory for media files
        true, // strict_mode
    ).map_err(WrappedPdlError)?;

    if let Some(message) = prompt.last_mut() {
        debug_assert_eq!(message.role, Role::User);
        let content = tokens.iter().map(
            |content| MessageContent::from(content.clone())
        ).collect::<Vec<MessageContent>>();

        message.content = content;
    }

    else {
        unreachable!()
    }

    let messages = prompt;
    let model = Model { name: api_config.model.clone(), ..Default::default() };
    let temperature = None;
    let frequency_penalty = None;
    let max_tokens = None;
    let timeout = api_config.timeout;
    let max_retry = api_config.max_retry;
    let sleep_between_retries = api_config.sleep_between_retries;
    let dump_api_usage_at = api_config.dump_api_usage_at(&PathBuf::from(&file), "create_chunk_from");
    let dump_pdl_at = api_config.create_pdl_path(&PathBuf::from(&file), "create_chunk_from").map(|path| path.to_string_lossy().into_owned());
    let dump_json_at = None;
    let schema = None;
    let schema_max_try = 0;

    let started_at = std::time::Instant::now();

    let mut data = vec![];
    let mut images = vec![];
    let mut char_len = 0;
    let mut image_count = 0;

    for r in tokens.iter() {
        match r {
            AtomicToken::String { data: s, char_len: n } => {
                data.push(s.clone());
                char_len += *n;
            },
            AtomicToken::Image(i) => {
                images.push(i.uid.clone());
                image_count += 1;
                data.push(format!("img_{}", i.uid));
            },
            _ => {} // Handle other AtomicToken variants if necessary
        }
    }

    let data = data.concat();

    let (_response_text, title, summary) = chunk_creation_utils::send_and_validate_chunk_response(
        messages,
        model,
        temperature,
        frequency_penalty,
        max_tokens,
        timeout,
        max_retry,
        sleep_between_retries,
        dump_api_usage_at,
        dump_pdl_at,
        dump_json_at,
        schema,
        schema_max_try,
        started_at,
        config.min_summary_len,
        config.max_summary_len,
        char_len,
        image_count,
        data.clone(),
        dry_run_llm,
    ).await?;

    let mut hasher = Sha3_256::new();
    hasher.update(data.as_bytes());
    hasher.update(title.as_bytes());
    hasher.update(summary.as_bytes());
    let uid = hasher.finalize();

    let uid_array: [u8; 32] = uid.into();
    Ok(Chunk {
        data,
        images,
        char_len,
        image_count,
        title,
        summary,
        file: normalize(&file)?,
        index: file_index,
        uid: uid_from_hash(&uid_array)?,
        build_info,
        timestamp: 0, // Placeholder, needs to be set correctly
        searchable: true, // Placeholder, needs to be set correctly
        
        source: ChunkSource::default(), // Placeholder
    })
}
