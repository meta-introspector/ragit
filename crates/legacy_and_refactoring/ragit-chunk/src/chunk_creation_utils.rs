use anyhow::Result;
use ragit_api::Request;
use ragit_model::Model;
use ragit_types::ApiError as Error;
use ragit_types::pdl_types::{Message, MessageContent, Role};
use serde_json::Value as JsonValue;
use regex::Regex;
use std::time::Instant;
//use ragit_types::Schema;
use ragit_types::schema::Schema;

pub async fn send_and_validate_chunk_response(
    mut messages: Vec<Message>,
    model: Model,
    temperature: Option<f64>,
    frequency_penalty: Option<f64>,
    max_tokens: Option<usize>,
    timeout: Option<u64>,
    max_retry: usize,
    sleep_between_retries: u64,
    dump_api_usage_at: Option<ragit_types::AuditRecordAt>,
    dump_pdl_at: Option<String>,
    dump_json_at: Option<String>,
    schema: Option<Schema>,
    schema_max_try: usize,
    _started_at: Instant,
    config_min_summary_len: usize,
    config_max_summary_len: usize,
    char_len: usize,
    image_count: usize,
    data: String,
    dry_run_llm: bool,
) -> Result<(String, String, String), Error> { // (response_text, title, summary)
    let json_regex = Regex::new(r"(?s)[^{}]*({.*})[^{}]*").unwrap();
    let mut mistakes = 0;

    loop {
        let error_message;

        let response_text = if dry_run_llm {
            use tokio::fs::OpenOptions;
            use tokio::io::AsyncWriteExt;
            use std::path::PathBuf;

            let log_path = PathBuf::from("/data/data/com.termux/files/home/storage/github/ragit/tmp_bootstrap/llm_queries.md");
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&log_path)
                .await
                .expect("Failed to open llm_queries.md");

            file.write_all(b"--- New LLM Query ---\n").await.expect("Failed to write to llm_queries.md");
            for message in &messages {
                file.write_all(format!("Role: {:?}\n", message.role).as_bytes()).await.expect("Failed to write to llm_queries.md");
                for content in &message.content {
                    match content {
                        MessageContent::String(s) => file.write_all(format!("Content: {}\n", s).as_bytes()).await.expect("Failed to write to llm_queries.md"),
                        _ => file.write_all(b"Content: (Non-string content)\n").await.expect("Failed to write to llm_queries.md"),
                    }
                }
                file.write_all(b"\n").await.expect("Failed to write to llm_queries.md");
            }
            file.write_all(b"---------------------\n\n").await.expect("Failed to write to llm_queries.md");

            // Return dummy values for dry run
            String::from("{\"title\": \"dummy title\", \"summary\": \"dummy summary\"}")
        } else {
            let response = Request::ChatRequest {
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
            response.get_message(0).unwrap().to_string()
        };

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

                                if summary_len < config_min_summary_len && char_len + image_count * 2 > config_min_summary_len * 2 {
                                    error_message = format!("Your summary is too short. Make sure that it's at least {} characters long.", config_min_summary_len);
                                }

                                else if summary_len > config_max_summary_len {
                                    error_message = format!("Your summary is too long. Make sure that it's less than {} characters long.", config_max_summary_len);
                                }

                                else if title.contains("\n") || summary.contains("\n") {
                                    error_message = format!("Your output has a correct schema, but please don't include newline characters in your output.");
                                }

                                else {
                                    return Ok((response_text.to_string(), title.to_string(), summary.to_string()));
                                }
                            },
                            _ => {
                                error_message = String::from("Give me a json object with 2 keys: \"title\" and \"summary\". Make sure that both are string.");
                            },
                        },
                        _ => {
                            error_message = String::from("Give me a json object with 2 keys: \"title\" and \"\"summary\".");
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
            return Ok((
                response_text.to_string(),
                String::from("untitled"),
                data_chars[..((config_min_summary_len + config_max_summary_len) / 2).min(data_chars.len())].iter().collect::<String>().replace("\n", " "),
            ));
        }

        messages.push(Message {
            role: Role::Assistant,
            content: vec![MessageContent::String(response_text.to_string())],
        });
        messages.push(Message {
            role: Role::User,
            content: vec![MessageContent::String(error_message)],
        });
    }
}
