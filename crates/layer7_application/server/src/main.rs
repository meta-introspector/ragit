#![recursion_limit = "256"]

use ragit_fs::{WriteMode, exists, initialize_log, remove_dir_all, write_log, write_string};
use ragit_server::CONFIG;
use ragit_server::cli::{CliCommand, RunArgs, parse_cli_args};
use ragit_server::config::Config;
use ragit_server::methods::*;
use ragit_server::models::ai_model::initialize_ai_models;
use ragit_server::utils::fetch_form_data;
use serde_json::Value;
use std::collections::HashMap;
use std::io::Write;
use warp::Filter;
use warp::filters::multipart::FormData;
use warp::http::status::StatusCode;
use warp::reply::with_status;

#[tokio::main]
async fn main() {
    let args = match parse_cli_args(std::env::args().collect::<Vec<_>>()) {
        Ok(command) => match command {
            CliCommand::Run(args) => args,
            CliCommand::DropAll(ref args) | CliCommand::TruncateAll(ref args) => {
                if !args.force {
                    println!("Are you sure?");
                    print!(">>> ");
                    let mut s = String::new();
                    std::io::stdout().flush().unwrap();
                    std::io::stdin().read_line(&mut s).unwrap();

                    if !s.starts_with("y") {
                        return;
                    }
                }

                let repo_data_dir = match &args.repo_data_dir {
                    Some(dir) => dir.to_string(),
                    None => {
                        let config = Config::default();
                        config.repo_data_dir
                    }
                };

                if exists(&repo_data_dir) {
                    remove_dir_all(&repo_data_dir).unwrap();
                }

                if let CliCommand::DropAll(_) = command {
                    drop_all().await.unwrap();
                } else {
                    truncate_all().await.unwrap();
                }

                return;
            }
        },
        Err(e) => {
            eprintln!("{e:?}");
            std::process::exit(1);
        }
    };
    initinalize_server(&args).await;
    let config = CONFIG.get().unwrap();

    write_log("server", "hello from ragit-server!");

    let get_index_handler = warp::get()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("index"))
        .and(warp::path::end())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(get_index);

    let get_config_handler = warp::get()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("config"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(get_config);

    let get_prompt_handler = warp::get()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("prompt"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(get_prompt);

    let get_uid_handler = warp::get()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("uid"))
        .and(warp::path::end())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(get_uid);

    let get_chunk_count_handler = warp::get()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("chunk-count"))
        .and(warp::path::end())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(get_chunk_count);

    let get_chunk_list_handler = warp::get()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("chunk-list"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(get_chunk_list);

    let get_chunk_list_all_handler = warp::get()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("chunk-list"))
        .and(warp::path::end())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(get_chunk_list_all);

    let get_chunk_handler = warp::get()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("chunk"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(get_chunk);

    let get_image_list_handler = warp::get()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("image-list"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(get_image_list);

    let get_image_handler = warp::get()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("image"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(get_image);

    let get_image_desc_handler = warp::get()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("image-desc"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(get_image_desc);

    let get_cat_file_handler = warp::get()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("cat-file"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(get_cat_file);

    let get_content_handler = warp::get()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("content"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(get_content);

    let get_archive_list_handler = warp::get()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("archive-list"))
        .and(warp::path::end())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(get_archive_list);

    let get_archive_handler = warp::get()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("archive"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(get_archive);

    let get_meta_handler = warp::get()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("meta"))
        .and(warp::path::end())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(get_meta);

    let get_meta_by_key_handler = warp::get()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("meta"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(get_meta_by_key);

    let get_version_handler = warp::get()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("version"))
        .and(warp::path::end())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(get_version);

    let get_server_version_handler = warp::get()
        .and(warp::path("version"))
        .and(warp::path::end())
        .map(get_server_version);

    let get_user_list_handler = warp::get()
        .and(warp::path("user-list"))
        .and(warp::path::end())
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(get_user_list);

    let get_user_handler = warp::get()
        .and(warp::path("user-list"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(get_user);

    let create_user_handler = warp::post()
        .and(warp::path("user-list"))
        .and(warp::path::end())
        .and(warp::body::json::<Value>())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(create_user);

    let get_repo_list_handler = warp::get()
        .and(warp::path("repo-list"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(get_repo_list);

    let get_repo_handler = warp::get()
        .and(warp::path("repo-list"))
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(get_repo);

    let get_user_ai_model_list_handler = warp::get()
        .and(warp::path("user-list"))
        .and(warp::path::param::<String>())
        .and(warp::path("ai-model-list"))
        .and(warp::path::end())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(get_user_ai_model_list);

    let put_user_ai_model_list_handler = warp::put()
        .and(warp::path("user-list"))
        .and(warp::path::param::<String>())
        .and(warp::path("ai-model-list"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(put_user_ai_model_list);

    let create_repo_handler = warp::post()
        .and(warp::path("repo-list"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(warp::body::json::<Value>())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(create_repo);

    let put_repo_handler = warp::put()
        .and(warp::path("repo-list"))
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(warp::body::json::<Value>())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(put_repo);

    let get_chat_handler = warp::get()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("chat"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(get_chat);

    let get_chat_list_handler = warp::get()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("chat-list"))
        .and(warp::path::end())
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(get_chat_list);

    let get_file_content_handler = warp::get()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("file-content"))
        .and(warp::path::end())
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(get_file_content);

    let post_begin_push_handler = warp::post()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("begin-push"))
        .and(warp::path::end())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(post_begin_push);

    let post_archive_handler = warp::post()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("archive"))
        .and(warp::path::end())
        .and(warp::multipart::form())
        .then(post_archive);

    let post_finalize_push_handler = warp::post()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("finalize-push"))
        .and(warp::path::end())
        .and(warp::body::bytes())
        .then(post_finalize_push);

    let create_chat_handler = warp::post()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("chat-list"))
        .and(warp::path::end())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(create_chat);

    let post_chat_handler_multipart_form = warp::post()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("chat"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(warp::multipart::form())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(
            async |user: String,
                   repo: String,
                   chat_id: String,
                   form: FormData,
                   api_key: Option<String>| {
                match fetch_form_data(form).await.handle_error(400) {
                    Ok(form) => post_chat(user, repo, chat_id, form, api_key).await,
                    Err((code, _)) => Box::new(with_status(
                        String::new(),
                        StatusCode::from_u16(code).unwrap(),
                    )),
                }
            },
        );

    // TODO: I want ragit-server to accept multipart requests and urlencoded requests. The problem is that
    //       `warp::body::form::<HashMap<String, String>>` can catch `application/x-www-form-urlencoded`,
    //       but `warp::body::form::<HashMap<String, Vec<u8>>>` cannot. Is this an upstream issue?
    let post_chat_handler_body_form = warp::post()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("chat"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(warp::body::form::<HashMap<String, String>>())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(
            async |user: String,
                   repo: String,
                   chat_id: String,
                   form: HashMap<String, String>,
                   api_key: Option<String>| {
                post_chat(
                    user,
                    repo,
                    chat_id,
                    form.into_iter()
                        .map(|(key, value)| (key, value.as_bytes().to_vec()))
                        .collect(),
                    api_key,
                )
                .await
            },
        );

    let post_chat_handler_json_form = warp::post()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("chat"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(warp::body::json::<Value>())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(
            async |user: String,
                   repo: String,
                   chat_id: String,
                   form: Value,
                   api_key: Option<String>| {
                match serde_json::from_value::<HashMap<String, String>>(form).handle_error(400) {
                    Ok(form) => {
                        post_chat(
                            user,
                            repo,
                            chat_id,
                            form.into_iter()
                                .map(|(key, value)| (key, value.as_bytes().to_vec()))
                                .collect(),
                            api_key,
                        )
                        .await
                    }
                    Err((code, _)) => Box::new(with_status(
                        String::new(),
                        StatusCode::from_u16(code).unwrap(),
                    )),
                }
            },
        );

    let get_traffic_handler = warp::get()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("traffic"))
        .and(warp::header::optional::<String>("x-api-key"))
        .then(get_traffic);

    let post_build_search_index_handler = warp::post()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("build-search-index"))
        .and(warp::path::end())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(post_build_search_index);

    let get_api_key_list_handler = warp::get()
        .and(warp::path("user-list"))
        .and(warp::path::param::<String>())
        .and(warp::path("api-key-list"))
        .and(warp::path::end())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(get_api_key_list);

    let create_api_key_handler = warp::post()
        .and(warp::path("user-list"))
        .and(warp::path::param::<String>())
        .and(warp::path("api-key-list"))
        .and(warp::path::end())
        .and(warp::body::json::<Value>())
        .then(create_api_key);

    let search_handler = warp::get()
        .and(warp::path::param::<String>())
        .and(warp::path::param::<String>())
        .and(warp::path("search"))
        .and(warp::path::end())
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(search);

    let get_ai_model_list_handler = warp::get()
        .and(warp::path("ai-model-list"))
        .and(warp::path::end())
        .and(warp::query::<HashMap<String, String>>())
        .then(get_ai_model_list);

    let put_ai_model_list_handler = warp::put()
        .and(warp::path("ai-model-list"))
        .and(warp::path::end())
        .and(warp::body::json::<Value>())
        .and(warp::header::optional::<String>("x-api-key"))
        .then(put_ai_model_list);

    let get_health_handler = warp::get().and(warp::path("health")).map(get_health);

    let not_found_handler = warp::get().map(not_found);

    warp::serve(
        get_server_version_handler
            .or(get_user_list_handler)
            .or(get_repo_list_handler)
            .or(get_user_handler)
            .or(get_user_ai_model_list_handler)
            .or(put_user_ai_model_list_handler)
            .or(get_repo_handler)
            .or(get_index_handler)
            .or(get_config_handler)
            .or(get_prompt_handler)
            .or(get_uid_handler)
            .or(get_chunk_count_handler)
            .or(get_chunk_list_handler)
            .or(get_chunk_list_all_handler)
            .or(get_chunk_handler)
            .or(get_image_list_handler)
            .or(get_image_handler)
            .or(get_image_desc_handler)
            .or(get_cat_file_handler)
            .or(get_content_handler)
            .or(get_archive_list_handler)
            .or(get_archive_handler)
            .or(get_meta_handler)
            .or(get_meta_by_key_handler)
            .or(get_version_handler)
            .or(get_chat_handler)
            .or(get_chat_list_handler)
            .or(get_file_content_handler)
            .or(create_user_handler)
            .or(create_repo_handler)
            .or(put_repo_handler)
            .or(post_begin_push_handler)
            .or(post_archive_handler)
            .or(post_finalize_push_handler)
            .or(post_chat_handler_multipart_form)
            .or(post_chat_handler_body_form)
            .or(post_chat_handler_json_form)
            .or(create_chat_handler)
            .or(get_traffic_handler)
            .or(post_build_search_index_handler)
            .or(get_api_key_list_handler)
            .or(create_api_key_handler)
            .or(search_handler)
            .or(get_ai_model_list_handler)
            .or(put_ai_model_list_handler)
            .or(get_health_handler)
            .or(not_found_handler)
            .with(warp::log::custom(|info| {
                let headers = info.request_headers();

                write_log(
                    &info
                        .remote_addr()
                        .map(|remote_addr| remote_addr.to_string())
                        .unwrap_or_else(|| String::from("NO REMOTE ADDR")),
                    &format!(
                        "{:4} {:16} {:4} {headers:?}",
                        info.method().as_str(),
                        info.path(),
                        info.status().as_u16(),
                    ),
                );
            })),
    )
    .run(([0, 0, 0, 0], args.port_number.unwrap_or(config.port_number)))
    .await;
}

// It sets global value `crate::CONFIG` and initialize ai models (if needed).
async fn initinalize_server(args: &RunArgs) {
    let pool = get_pool().await;
    sqlx::migrate!().run(pool).await.unwrap();
    let config_file = args
        .config_file
        .clone()
        .unwrap_or(String::from("./config.json"));

    if !exists(&config_file) {
        if args.force_default_config {
            let config = Config::default();
            write_string(
                &config_file,
                &serde_json::to_string(&config).unwrap(),
                WriteMode::CreateOrTruncate,
            )
            .unwrap();
        } else {
            println!(
                "Config file `{config_file}` does not exist. Would you like to create a new one?"
            );
            println!("");
            println!("1) Create a default config file at `{config_file}`.");
            println!("2) Let me use a GUI to create a config file.");
            println!("3) I don't know what you're talking about. Please help me.");

            loop {
                print!(">>> ");
                let mut s = String::new();
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut s).unwrap();

                match s.get(0..1) {
                    Some("1") => {
                        let config = Config::default();
                        write_string(
                            &config_file,
                            &serde_json::to_string(&config).unwrap(),
                            WriteMode::CreateOrTruncate,
                        )
                        .unwrap();
                    }
                    Some("2") => todo!(),
                    Some("3") => todo!(),
                    _ => {
                        println!("Just say 1, 2 or 3.");
                        continue;
                    }
                }

                break;
            }
        }
    }

    let config = match Config::load_from_file(&config_file) {
        Ok(c) => c,
        Err(e) => panic!("Failed to load config file: `{e:?}`."),
    };

    if !args.quiet {
        initialize_log(
            config.log_file.clone(),
            args.verbose || config.dump_log_to_stdout,
            false, // dump_to_stderr
            true,  // keep_previous_file
        )
        .unwrap();
    }

    if exists(&config.push_session_dir) {
        remove_dir_all(&config.push_session_dir).unwrap();
    }

    initialize_ai_models(pool).await.unwrap();
    CONFIG.set(config).unwrap();
}
