use std::path::PathBuf;
use crate::prelude::{Message, Model, Schema, ApiProvider};
use ragit_types::ApiError as Error;
use ragit_session_query::response::Response;
use ragit_types::AuditRecordAt;
use crate::audit::{dump_api_usage, dump_pdl};
use crate::message::{message_contents_to_json_array, message_to_json};
use crate::rate_limit::RateLimiter;
use async_std::task;
use chrono::Local;
use ragit_fs::{create_dir_all, exists_str, join, write_log, write_string, WriteMode};
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};
use std::time::{Duration, Instant};
use ragit_types::pdl_types::Role;
use ragit_model::ModelRaw;
use crate::map_serde_json_error;

#[derive(Debug)]
pub enum Request {
    BuildChunks { file: PathBuf },
    Kill,
    ChatRequest {
        messages: Vec<Message>,
        model: Model,
        temperature: Option<f64>,
        frequency_penalty: Option<f64>,
        max_tokens: Option<usize>,
        timeout: Option<u64>,
        max_retry: usize,
        sleep_between_retries: u64,
        dump_api_usage_at: Option<AuditRecordAt>,
        dump_pdl_at: Option<String>,
        dump_json_at: Option<String>,
        schema: Option<Schema>,
        schema_max_try: usize,
    },
}

impl Request {
    pub async fn send(&self) -> Result<Response, Error> {
        match self {
            Request::ChatRequest {
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
                schema: _,
                schema_max_try: _,
            } => {
                let started_at = Instant::now();
                let client = reqwest::Client::new();
                let mut curr_error = Error::NoTry;

                let mut rate_limiter = RateLimiter::new(&model, 0.9);

                let post_url = model.get_api_url()?;
                let body = Request::build_json_body(
                    messages,
                    model,
                    temperature,
                    frequency_penalty,
                    max_tokens,
                );

                if let Err(e) = Request::dump_json(&body, "request", dump_json_at) {
                    write_log(
                        "dump_json",
                        &format!("dump_json(\"request\", ..) failed with {e:?}"),
                    );
                }
                let api_provider =
                    ApiProvider::parse(&model.api_provider_name, &model.api_url)?;
                if model.api_provider_name == "Test" {
                    let response =
                        model.test_model.as_ref().unwrap().get_dummy_response(&messages)?;

                    if let Some(key) = dump_api_usage_at {
                        if let Err(e) = dump_api_usage(
                            key,
                            0,
                            0,
                            model.dollars_per_1b_input_tokens,
                            model.dollars_per_1b_output_tokens,
                            false,
                        ) {
                            write_log(
                                "dump_api_usage",
                                &format!("dump_api_usage({key:?}, ..) failed with {e:?}"),
                            );
                        }
                    }

                    if let Some(path) = dump_pdl_at {
                        if let Err(e) = dump_pdl(
                            &messages,
                            &response,
                            &None,
                            path,
                            String::from("model: dummy, input_tokens: 0, output_tokens: 0, took: 0ms"),
                        ) {
                            write_log(
                                "dump_pdl",
                                &format!("dump_pdl({path:?}, ..) failed with {e:?}"),
                            );

                            // TODO: should it return an error?
                            //       the api call was successful
                        }
                    }

                    return Ok(Response::dummy(response));
                }

                let body = map_serde_json_error(serde_json::to_string(&body))?;
                let api_key = model.get_api_key()?;
                write_log(
                    "chat_request::send",
                    &format!(
                        "entered chat_request::send() with {} bytes, model: {}",
                        body.len(),
                        model.name
                    ),
                );

                for _ in 0..(max_retry + 1) {
                    let delay = rate_limiter.check_and_throttle().unwrap();
                    task::sleep(delay).await;

                    let mut request = client
                        .post(&post_url)
                        .header(reqwest::header::CONTENT_TYPE, "application/json")
                        .body(body.clone());

                    match model.api_provider_name.as_str() {
                        "Anthropic" => {
                            request = request
                                .header("x-api-key", api_key.clone())
                                .header("anthropic-version", "2023-06-01");
                        }
                        "Google" => {}
                        _ if !api_key.is_empty() => {
                            request = request.bearer_auth(api_key.clone());
                        }
                        _ => {}
                    }

                    if let Some(t) = timeout {
                        request = request.timeout(Duration::from_millis(*t));
                    }

                    write_log("chat_request::send", "a request sent");
                    let response = request.send().await;
                    write_log("chat_request::send", "got a response from a request");

                    match response {
                        Ok(response) => match response.status().as_u16() {
                            200 => match response.text().await {
                                Ok(text) => {
                                    match map_serde_json_error(serde_json::from_str::<Value>(&text)) {
                                        Ok(v) => match Request::dump_json(&v, "response", dump_json_at) {
                                            Err(e) => {
                                                write_log(
                                                    "dump_json",
                                                    &format!(
                                                        "dump_json(\"response\", ..) failed with {e:?}"
                                                    ),
                                                );
                                            }
                                            Ok(_) => {}
                                        },
                                        Err(e) => {
                                            write_log(
                                                "dump_json",
                                                &format!("dump_json(\"response\", ..) failed with {e:?}"),
                                            );
                                        }
                                    }

                                    match Response::from_str(&text, &api_provider) {
                                        Ok(result) => {
                                            rate_limiter
                                                .record_usage(1, result.get_output_token_count() as u32);
                                            if let Some(key) = dump_api_usage_at {
                                                if let Err(e) = dump_api_usage(
                                                    key,
                                                    result.get_prompt_token_count() as u64,
                                                    result.get_output_token_count() as u64,
                                                    model.dollars_per_1b_input_tokens,
                                                    model.dollars_per_1b_output_tokens,
                                                    false,
                                                ) {
                                                    write_log(
                                                        "dump_api_usage",
                                                        &format!(
                                                            "dump_api_usage({key:?}, ..) failed with {e:?}"
                                                        ),
                                                    );
                                                }
                                            }

                                            if let Some(path) = dump_pdl_at {
                                                if let Err(e) = dump_pdl(
                                                    &messages,
                                                    &result
                                                        .get_message(0)
                                                        .map(|m| m.to_string())
                                                        .unwrap_or(String::new()),
                                                    &result.get_reasoning(0).map(|m| m.to_string()),
                                                    path,
                                                    format!(
                                                        "model: {}, input_tokens: {}, output_tokens: {}, took: {}ms",
                                                        model.name,
                                                        result.get_prompt_token_count(),
                                                        result.get_output_token_count(),
                                                        Instant::now()
                                                            .duration_since(started_at.clone())
                                                            .as_millis(),
                                                    ),
                                                ) {
                                                    write_log(
                                                        "dump_pdl",
                                                        &format!(
                                                            "dump_pdl({path:?}, ..) failed with {e:?}"
                                                        ),
                                                    );

                                                    // TODO: should it return an error?
                                                    //       the api call was successful
                                                }
                                            }

                                            return Ok(result);
                                        }
                                        Err(e) => {
                                            write_log(
                                                "Response::from_str",
                                                &format!("Response::from_str(..) failed with {e:?}"),
                                            );
                                            curr_error = e;
                                        }
                                    }
                                }
                                Err(e) => {
                                    write_log(
                                        "response.text()",
                                        &format!("response.text() failed with {e:?}"),
                                    );
                                    curr_error = Error::ReqwestError(e.to_string());
                                }
                            },
                            status_code => {
                                curr_error = Error::ServerError {
                                    status_code,
                                    body: response.text().await.unwrap_or_default(),
                                };

                                if let Some(path) = dump_pdl_at {
                                    if let Err(e) = dump_pdl(
                                        &messages,
                                        "",
                                        &None,
                                        path,
                                        format!("{}# error: {curr_error:?} #{}", '{', "}"),
                                    ) {
                                        write_log(
                                            "dump_pdl",
                                            &format!("dump_pdl({path:?}, ..) failed with {e:?}"),
                                        );
                                    }
                                }

                                // There are 2 cases.
                                // 1. `self.model.can_read_images` is false, but it can actually read images.
                                //   - Maybe `self.model` is outdated.
                                //   - That's why it tries once even though there is an image.
                                // 2. `self.model.can_read_images` is false, and it cannot read images.
                                //   - There's no point in retrying, so it just escapes immediately with a better error.
                                if !model.can_read_images
                                    && messages.iter().any(|message| message.has_image())
                                {
                                    return Err(Error::CannotReadImage(model.name.clone()));
                                }
                            }
                        },
                        Err(e) => {
                            write_log(
                                "request.send().await",
                                &format!("request.send().await failed with {e:?}"),
                            );
                            curr_error = Error::ReqwestError(e.to_string());
                        }
                    }

                    task::sleep(Duration::from_millis(*sleep_between_retries)).await
                }

                Err(curr_error)
            }
            _ => Err(Error::InvalidRequest), // Handle other Request variants
        }
    }

    pub async fn send_and_validate<T: DeserializeOwned>(
        &self,
        default: T,
    ) -> Result<T, Error> {
        match self {
            Request::ChatRequest {
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
            } => {
                let mut state_messages = messages.clone();
                for _ in 0..*schema_max_try {
                    let response = Request::ChatRequest {
                        messages: state_messages.clone(),
                        model: model.clone(),
                        temperature: *temperature,
                        frequency_penalty: *frequency_penalty,
                        max_tokens: *max_tokens,
                        timeout: *timeout,
                        max_retry: *max_retry,
                        sleep_between_retries: *sleep_between_retries,
                        dump_api_usage_at: dump_api_usage_at.clone(),
                        dump_pdl_at: dump_pdl_at.clone(),
                        dump_json_at: dump_json_at.clone(),
                        schema: schema.clone(),
                        schema_max_try: *schema_max_try,
                    }
                    .send()
                    .await?;
                    let response = response.get_message(0).unwrap();

                    match schema.as_ref().unwrap().validate(&response) {
                        Ok(v) => {
                            return Ok(map_serde_json_error(serde_json::from_value::<T>(v))?);
                        }
                        Err(error_message) => {
                            state_messages.push(Message::simple_message(
                                Role::Assistant,
                                response.to_string(),
                            ));
                            state_messages.push(Message::simple_message(Role::User, error_message));
                        }
                    }
                }
                Ok(default)
            }
            _ => Err(Error::InvalidRequest), // Handle other Request variants
        }
    }

    fn build_json_body(
        messages: &Vec<Message>,
        model: &Model,
        temperature: &Option<f64>,
        frequency_penalty: &Option<f64>,
        max_tokens: &Option<usize>,
    ) -> Value {
        let api_provider =
            ApiProvider::parse(&model.api_provider_name, &model.api_url).unwrap();
        match model.api_provider_name.as_str() {
            "Google" => {
                let mut result = Map::new();
                let mut contents = vec![];
                let mut system_prompt = vec![];

                for message in messages.iter() {
                    if message.role == Role::System {
                        match message_contents_to_json_array(&message.content, &api_provider) {
                            Value::Array(parts) => {
                                system_prompt.push(parts);
                            }
                            _ => unreachable!(),
                        }
                    } else {
                        contents.push(message_to_json(message, &api_provider));
                    }
                }

                if !system_prompt.is_empty() {
                    let parts = system_prompt.concat();
                    let mut system_prompt = Map::new();
                    system_prompt.insert(String::from("parts"), parts.into());
                    result.insert(String::from("system_instruction"), system_prompt.into());
                }

                // TODO: temperature

                result.insert(String::from("contents"), contents.into());
                result.into()
            }
            "OpenAi" | "Cohere" => {
                let mut result = Map::new();
                result.insert(String::from("model"), model.api_name.clone().into());
                let mut messages_json = vec![];

                for message in messages.iter() {
                    messages_json.push(message_to_json(message, &api_provider));
                }

                result.insert(String::from("messages"), messages_json.into());

                if let Some(temp) = temperature {
                    result.insert(String::from("temperature"), (*temp).into());
                }

                if let Some(freq_penalty) = frequency_penalty {
                    result.insert(String::from("frequency_penalty"), (*freq_penalty).into());
                }

                if let Some(max_tok) = max_tokens {
                    result.insert(String::from("max_tokens"), (*max_tok).into());
                }

                result.into()
            }
            "Anthropic" => {
                let mut result = Map::new();
                result.insert(String::from("model"), model.api_name.clone().into());
                let mut messages_json = vec![];
                let mut system_prompt_str = vec![];

                for message in messages.iter() {
                    if message.role == Role::System {
                        system_prompt_str.push(message.content[0].unwrap_str().to_string());
                    } else {
                        messages_json.push(message_to_json(message, &api_provider));
                    }
                }

                let system_prompt_str = system_prompt_str.concat();

                if !system_prompt_str.is_empty() {
                    result.insert(String::from("system"), system_prompt_str.into());
                }

                result.insert(String::from("messages"), messages_json.into());

                if let Some(temp) = temperature {
                    result.insert(String::from("temperature"), (*temp).into());
                }

                if let Some(freq_penalty) = frequency_penalty {
                    result.insert(String::from("frequency_penalty"), (*freq_penalty).into());
                }

                // it's a required field
                result.insert(
                    String::from("max_tokens"),
                    max_tokens.unwrap_or(2048).into(),
                );

                result.into()
            }
            _ => Value::Null,
        }
    }

    fn dump_json(j: &Value, header: &str, dump_json_at: &Option<String>) -> Result<(), Error> {
        if let Some(dir) = dump_json_at {
            if !exists_str(dir) {
                create_dir_all(dir)?;
            }

            let path = join(
                &dir,
                &format!("{}-{}.json", header, Local::now().to_rfc3339()),
            )?;
            write_string(
                &path,
                &map_serde_json_error(serde_json::to_string_pretty(j))?,
                WriteMode::AlwaysCreate,
            )?;
        }

        Ok(())
    }
}

impl Default for Request {
    fn default() -> Self {
        Request::ChatRequest {
            messages: vec![],
            model: (&ModelRaw::llama_70b()).try_into().unwrap(),
            temperature: None,
            frequency_penalty: None,
            max_tokens: None,
            timeout: Some(20_000),
            max_retry: 2,
            sleep_between_retries: 6_000,
            dump_api_usage_at: None,
            dump_pdl_at: None,
            dump_json_at: None,
            schema: None,
            schema_max_try: 3,
        }
    }
}
