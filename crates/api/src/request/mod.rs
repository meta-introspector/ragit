use crate::audit::{dump_api_usage, dump_pdl};
use crate::message::{message_contents_to_json_array, message_to_json};
use crate::prelude::*;
use crate::rate_limit::RateLimiter;
use async_std::task;
use chrono::Local;
use ragit_fs::{create_dir_all, exists_str, join, write_log, write_string, WriteMode};
use ragit_types::AuditRecordAt;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct Request {
    pub messages: Vec<Message>,
    pub model: Model,
    pub temperature: Option<f64>,
    pub frequency_penalty: Option<f64>,
    pub max_tokens: Option<usize>,

    /// milliseconds
    pub timeout: Option<u64>,

    /// It tries 1 + max_retry times.
    pub max_retry: usize,

    /// milliseconds
    pub sleep_between_retries: u64,
    pub dump_api_usage_at: Option<AuditRecordAt>,

    /// It dumps the AI conversation in pdl format. See <https://crates.io/crates/ragit-pdl> to read about pdl.
    pub dump_pdl_at: Option<String>,

    /// It's a directory, not a file. If given, it dumps `dir/request-<timestamp>.json` and `dir/response-<timestamp>.json`.
    pub dump_json_at: Option<String>,

    /// It can force LLMs to create a json output with a given schema.
    /// You have to call `send_and_validate` instead of `send` if you want
    /// to force the schema.
    pub schema: Option<Schema>,

    /// If LLMs fail to generate a valid schema `schema_max_try` times,
    /// it returns a default value. If it's 0, it wouldn't call LLM at all!
    pub schema_max_try: usize,
}

impl Request {
    pub fn is_valid(&self) -> bool {
        self.messages.len() > 1
            && self.messages.len() & 1 == 0 // the last message must be user's
            && self.messages[0].is_valid_system_prompt() // I'm not sure whether all the models require the first message to be a system prompt. but it would be safer to guarantee that
            && {
                let mut flag = true;

                for (index, message) in self.messages[1..].iter().enumerate() {
                    if index & 1 == 0 && !message.is_user_prompt() {
                        flag = false;
                        break;
                    } else if index & 1 == 1 && !message.is_assistant_prompt() {
                        flag = false;
                        break;
                    }
                }

                flag
            }
    }

    /// It panics if its fields are not complete. If you're not sure, run `self.is_valid()` before sending a request.
    pub fn build_json_body(&self) -> Value {
        let api_provider =
            ApiProvider::parse(&self.model.api_provider_name, &self.model.get_api_url().ok()).unwrap();
        match self.model.api_provider_name.as_str() {
            "Google" => {
                let mut result = Map::new();
                let mut contents = vec![];
                let mut system_prompt = vec![];

                for message in self.messages.iter() {
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
                result.insert(String::from("model"), self.model.api_name.clone().into());
                let mut messages = vec![];

                for message in self.messages.iter() {
                    messages.push(message_to_json(message, &api_provider));
                }

                result.insert(String::from("messages"), messages.into());

                if let Some(temperature) = self.temperature {
                    result.insert(String::from("temperature"), temperature.into());
                }

                if let Some(frequency_penalty) = self.frequency_penalty {
                    result.insert(String::from("frequency_penalty"), frequency_penalty.into());
                }

                if let Some(max_tokens) = self.max_tokens {
                    result.insert(String::from("max_tokens"), max_tokens.into());
                }

                result.into()
            }
            "Anthropic" => {
                let mut result = Map::new();
                result.insert(String::from("model"), self.model.api_name.clone().into());
                let mut messages = vec![];
                let mut system_prompt = vec![];

                for message in self.messages.iter() {
                    if message.role == Role::System {
                        system_prompt.push(message.content[0].unwrap_str().to_string());
                    } else {
                        messages.push(message_to_json(message, &api_provider));
                    }
                }

                let system_prompt = system_prompt.concat();

                if !system_prompt.is_empty() {
                    result.insert(String::from("system"), system_prompt.into());
                }

                result.insert(String::from("messages"), messages.into());

                if let Some(temperature) = self.temperature {
                    result.insert(String::from("temperature"), temperature.into());
                }

                if let Some(frequency_penalty) = self.frequency_penalty {
                    result.insert(String::from("frequency_penalty"), frequency_penalty.into());
                }

                // it's a required field
                result.insert(
                    String::from("max_tokens"),
                    self.max_tokens.unwrap_or(2048).into(),
                );

                result.into()
            }
            _ => Value::Null,
        }
    }

    /// It panics if `schema` field is missing.
    /// It doesn't tell you whether the default value is used or not.
    pub async fn send_and_validate<T: DeserializeOwned>(&self, default: T) -> Result<T, Error> {
        let mut state = self.clone();
        let mut messages = self.messages.clone();

        for _ in 0..state.schema_max_try {
            state.messages = messages.clone();
            let response = state.send().await?;
            let response = response.get_message(0).unwrap();

            match state.schema.as_ref().unwrap().validate(&response) {
                Ok(v) => {
                    return Ok(serde_json::from_value::<T>(v).map_err(Error::JsonSerdeError)?);
                }
                Err(error_message) => {
                    messages.push(Message::simple_message(
                        Role::Assistant,
                        response.to_string(),
                    ));
                    messages.push(Message::simple_message(Role::User, error_message));
                }
            }
        }

        Ok(default)
    }

    /// NOTE: this function dies ocassionally, for no reason.
    ///
    /// It panics if its fields are not complete. If you're not sure, run `self.is_valid()` before sending a request.
    pub fn blocking_send(&self) -> Result<Response, Error> {
        futures::executor::block_on(self.send())
    }

    /// It panics if its fields are not complete. If you're not sure, run `self.is_valid()` before sending a request.
    pub async fn send(&self) -> Result<Response, Error> {
        let started_at = Instant::now();
        let client = reqwest::Client::new();
        let mut curr_error = Error::NoTry;

        let mut rate_limiter = RateLimiter::new(&self.model, 0.9);

        let post_url = self.model.get_api_url()?;
        let body = self.build_json_body();

        if let Err(e) = self.dump_json(&body, "request") {
            write_log(
                "dump_json",
                &format!("dump_json(\"request\", ..) failed with {e:?}"),
            );
        }
        let api_provider =
            ApiProvider::parse(&self.model.api_provider_name, &self.model.get_api_url().ok())?;
        if self.model.is_test_model() {
            let test_model = match self.model.name.as_str() {
                "dummy" => TestModel::Dummy,
                "stdin" => TestModel::Stdin,
                "error" => TestModel::Error,
                _ => return Err(Error::InvalidTestModel(self.model.name.clone())),
            };
            let response = test_model.get_dummy_response(&self.messages)?;

            if let Some(key) = &self.dump_api_usage_at {
                if let Err(e) = dump_api_usage(
                    key,
                    0,
                    0,
                    self.model.dollars_per_1b_input_tokens,
                    self.model.dollars_per_1b_output_tokens,
                    false,
                ) {
                    write_log(
                        "dump_api_usage",
                        &format!("dump_api_usage({key:?}, ..) failed with {e:?}"),
                    );
                }
            }

            if let Some(path) = &self.dump_pdl_at {
                if let Err(e) = dump_pdl(
                    &self.messages,
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

        let body = serde_json::to_string(&body).map_err(Error::JsonSerdeError)?;
        let api_key = self.model.get_api_key()?;
        write_log(
            "chat_request::send",
            &format!(
                "entered chat_request::send() with {} bytes, model: {}",
                body.len(),
                self.model.name
            ),
        );

        for _ in 0..(self.max_retry + 1) {
            let delay = rate_limiter.check_and_throttle().unwrap();
            task::sleep(delay).await;

            let mut request = client
                .post(&post_url)
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .body(body.clone());

            match self.model.api_provider_name.as_str() {
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

            if let Some(t) = self.timeout {
                request = request.timeout(Duration::from_millis(t));
            }

            write_log("chat_request::send", "a request sent");
            let response = request.send().await;
            write_log("chat_request::send", "got a response from a request");

            match response {
                Ok(response) => match response.status().as_u16() {
                    200 => match response.text().await {
                        Ok(text) => {
                            match serde_json::from_str::<Value>(&text) {
                                Ok(v) => match self.dump_json(&v, "response") {
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
                                    if let Some(key) = &self.dump_api_usage_at {
                                        if let Err(e) = dump_api_usage(
                                            key,
                                            result.get_prompt_token_count() as u64,
                                            result.get_output_token_count() as u64,
                                            self.model.dollars_per_1b_input_tokens,
                                            self.model.dollars_per_1b_output_tokens,
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

                                    if let Some(path) = &self.dump_pdl_at {
                                        if let Err(e) = dump_pdl(
                                            &self.messages,
                                            &result
                                                .get_message(0)
                                                .map(|m| m.to_string())
                                                .unwrap_or(String::new()),
                                            &result.get_reasoning(0).map(|m| m.to_string()),
                                            path,
                                            format!(
                                                "model: {}, input_tokens: {}, output_tokens: {}, took: {}ms",
                                                self.model.name,
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

                        if let Some(path) = &self.dump_pdl_at {
                            if let Err(e) = dump_pdl(
                                &self.messages,
                                "",
                                &None,
                                path,
                                format!("{}# error: {curr_error:?} #{}", '{', '}'),
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
                        if !self.model.can_read_images
                            && self.messages.iter().any(|message| message.has_image())
                        {
                            return Err(Error::CannotReadImage(self.model.name.clone()));
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

            task::sleep(Duration::from_millis(self.sleep_between_retries)).await
        }

        Err(curr_error)
    }

    fn dump_json(&self, j: &Value, header: &str) -> Result<(), Error> {
        if let Some(dir) = &self.dump_json_at {
            if !exists_str(dir) {
                create_dir_all(dir)?;
            }

            let path = join(
                &dir,
                &format!("{header}-{}.json", Local::now().to_rfc3339()),
            )?;
            write_string(
                &path,
                &serde_json::to_string_pretty(j)?,
                WriteMode::AlwaysCreate,
            )?;
        }

        Ok(())
    }
}

impl Default for Request {
    fn default() -> Self {
        Request {
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
