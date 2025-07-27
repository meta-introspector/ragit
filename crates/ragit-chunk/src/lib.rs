use ragit_types::AuditRecordAt;
use ragit_error_conversions::WrappedPdlError;
use ragit_types::build_config::BuildConfig;
use ragit_types::chunk::atomic_token::AtomicToken;
use ragit_types::api_config::ApiConfig;
use ragit_types::ApiError as Error;
use regex::Regex;
use serde_json::Value as JsonValue;
use sha3::{Digest, Sha3_256};
//use ragit_api::request::{ChatRequest, RecordAt};
//use ragit_pdl::messages_from_pdl;
use ragit_types::pdl_types::{Message, MessageContent, Role};
use ragit_fs::{normalize};
use ragit_types::chunk::chunk_struct::Chunk;
use ragit_types::chunk::chunk_source::ChunkSource;
use ragit_types::uid::Uid;
use std::str::FromStr;
use std::collections::HashMap;
use ragit_api::Request;
use ragit_model::Model;
use ragit_pdl::{Pdl, parse_pdl};
use std::path::PathBuf;
pub async fn create_chunk_from(
    tokens: &[AtomicToken],
    config: &BuildConfig,
    file: String,
    file_index: usize,
    api_config: &ApiConfig,
    pdl: &str,
    build_info: ragit_types::chunk::chunk_struct::ChunkBuildInfo,
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

    let mut messages = prompt;
    let mut model = Model { name: api_config.model.clone(), ..Default::default() };
    let mut temperature = None;
    let mut frequency_penalty = None;
    let mut max_tokens = None;
    let mut timeout = api_config.timeout;
    let mut max_retry = api_config.max_retry;
    let mut sleep_between_retries = api_config.sleep_between_retries;
    let mut dump_api_usage_at = api_config.dump_api_usage_at(&PathBuf::from(&file), "create_chunk_from");
    let mut dump_pdl_at = api_config.create_pdl_path(&PathBuf::from(&file), "create_chunk_from").map(|path| path.to_string_lossy().into_owned());
    let mut dump_json_at = None;
    let mut schema = None;
    let mut schema_max_try = 0;

    let mut response = ragit_api::Request::ChatRequest {
        messages: messages.clone(),
        model: model.clone(),
        temperature,
        frequency_penalty,
        max_tokens,
        timeout,
        max_retry,
        sleep_between_retries,
        dump_api_usage_at: dump_api_usage_at.clone(),
        dump_pdl_at: dump_pdl_at.clone(),
        dump_json_at: dump_json_at.clone(),
        schema: schema.clone(),
        schema_max_try,
    }.send().await?;
    let mut response_text = response.get_message(0).unwrap();
    let json_regex = Regex::new(r"(?s)[^{}]*({.*})[^{}]*").unwrap();

    let mut data = vec![];
    let mut images = vec![];
    let mut char_len = 0;
    let mut image_count = 0;
    let mut mistakes = 0;

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

    let (title, summary) = loop {
        let error_message;

        if let Some(cap) = json_regex.captures(&response_text) {
            let json_text = cap[1].to_string();

            match serde_json::from_str::<JsonValue>(&json_text) {
                Ok(j) => match j {
                    JsonValue::Object(obj) if obj.len() == 2 => match (
                        obj.get("title"), obj.get("summary")
                    ) {
                        (Some(title), Some(summary)) => match (title.as_str(), summary.as_str()) {
                            (Some(title), Some(summary)) => {
                                let summary_len = summary.chars().count();

                                if summary_len < config.min_summary_len && char_len + image_count * config.image_size > config.min_summary_len * 2 {
                                    error_message = format!("Your summary is too short. Make sure that it's at least {} characters long.", config.min_summary_len);
                                }

                                else if summary_len > config.max_summary_len {
                                    error_message = format!("Your summary is too long. Make sure that it's less than {} characters long.", config.max_summary_len);
                                }

                                else if title.contains("\n") || summary.contains("\n") {
                                    error_message = format!("Your output has a correct schema, but please don't include newline characters in your output.");
                                }

                                else {
                                    break (title.to_string(), summary.to_string());
                                }
                            },
                            _ => {
                                error_message = String::from("Give me a json object with 2 keys: \"title\" and \"summary\". Make sure that both are string.");
                            },
                        },
                        _ => {
                            error_message = String::from("Give me a json object with 2 keys: \"title\" and \"summary\".");
                        },
                    },
                    JsonValue::Object(_) => {
                        error_message = String::from("Please give me a json object that contains 2 keys: \"title\" and \"summary\". Don't add keys to give extra information, put all your information in those two fields.");
                    },
                    _ => {
                        error_message = String::from("Give me a json object with 2 keys: \"title\" and \"summary\".");
                    },
                },
                Err(_) => {
                    error_message = String::from("I cannot parse your output. It seems like your output is not a valid json. Please give me a valid json.");
                },
            }
        }

        else {
            error_message = String::from("I cannot find curly braces in your response. Please give me a valid json output.");
        }

        mistakes += 1;

        // if a model is too stupid, it cannot create title and summary
        if mistakes > 5 {
            let data_chars = data.chars().collect::<Vec<_>>();

            // default values
            break (
                String::from("untitled"),
                data_chars[..((config.min_summary_len + config.max_summary_len) / 2).min(data_chars.len())].into_iter().collect::<String>().replace("\n", " "),
            );
        }

        let mut chat_request = ragit_api::Request::ChatRequest {
            messages: messages.clone(),
            model: model.clone(),
            temperature,
            frequency_penalty,
            max_tokens,
            timeout,
            max_retry,
            sleep_between_retries,
            dump_api_usage_at: dump_api_usage_at.clone(),
            dump_pdl_at: dump_pdl_at.clone(),
            dump_json_at: dump_json_at.clone(),
            schema: schema.clone(),
            schema_max_try,
        };
        messages.push(Message {
            role: Role::Assistant,
            content: vec![MessageContent::String(response_text.to_string())],
        });
        messages.push(Message {
            role: Role::User,
            content: vec![MessageContent::String(error_message)],
        });
        response = ragit_api::Request::ChatRequest {
            messages: messages.clone(),
            model: model.clone(),
            temperature,
            frequency_penalty,
            max_tokens,
            timeout,
            max_retry,
            sleep_between_retries,
            dump_api_usage_at: dump_api_usage_at.clone(),
            dump_pdl_at: dump_pdl_at.clone(),
            dump_json_at: dump_json_at.clone(),
            schema: schema.clone(),
            schema_max_try,
        }.send().await?;
        response_text = response.get_message(0).unwrap();
    };
    let mut hasher = Sha3_256::new();
    hasher.update(data.as_bytes());
    hasher.update(title.as_bytes());
    hasher.update(summary.as_bytes());
    let uid = hasher.finalize();

    Ok(Chunk {
        data,
        images,
        char_len,
        image_count,
        title,
        summary,
        file: normalize(&file)?,
        index: file_index,
        uid: Uid::from_str(&format!("{:064x}", uid))?,
        build_info,
        timestamp: 0, // Placeholder, needs to be set correctly
        searchable: true, // Placeholder, needs to be set correctly
        muse_summaries: HashMap::new(), // Placeholder
        source: ChunkSource::default(), // Placeholder
    })
}
