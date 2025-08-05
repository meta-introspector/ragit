use serde_json::{Value, Map};
use chrono::Local;
use ragit_fs::{create_dir, read_string as fs_read_string, join as fs_join, exists};
use ragit_utils::path_utils::str_to_pathbuf;
use ragit_api::{get_model_by_name, Model, Request};
use ragit_model::model_raw::ModelRaw;
use ragit_types::api_config::ApiConfig;
use ragit_index_types::{index_struct::Index, load_mode::LoadMode};
use ragit_types::pdl_types::{Message, Role};
use ragit_pdl::parse_pdl_from_file::parse_pdl_from_file;
use ragit_pdl::schema::{parse::parse_schema, render::render_pdl_schema};
use ragit_utils::{cli_types::{ArgParser, ArgType, ArgCount}, doc_utils::get_doc_content, error::Error, project_root::find_root};
use tera;

pub async fn pdl_command_main(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .flag_with_default(&["--strict", "--no-strict"])
        .optional_arg_flag("--model", ArgType::String)
        .optional_arg_flag("--models", ArgType::Path)
        .optional_arg_flag("--context", ArgType::Path)
        .optional_arg_flag("--log", ArgType::Path)
        .optional_arg_flag("--schema", ArgType::String)
        .args(ArgType::Path, ArgCount::Exact(1))
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/pdl.txt"));
        return Ok(());
    }

    let index = find_root().map(|root_dir| Index::load(root_dir.into(), LoadMode::OnlyJson));
    let pdl_at = parsed_args.get_args_exact(1)?[0].clone();
    let strict_mode = parsed_args.get_flag(0).unwrap() == "--strict";
    let models = match parsed_args.arg_flags.get("--models") {
        Some(path) => {
            let m = fs_read_string(path)?;
            let models_raw = serde_json::from_str::<Vec<ModelRaw>>(&m)?;
            let mut models = Vec::with_capacity(models_raw.len());

            for model_raw in models_raw.iter() {
                models.push(Model::try_from(model_raw)?);
            }

            models
        }
        None => match &index {
            Ok(Ok(index)) => index.models.clone(),
            _ => {
                let models_raw = Index::get_initial_models()?;
                let mut models = Vec::with_capacity(models_raw.len());

                for model_raw in models_raw.iter() {
                    models.push(Model::try_from(model_raw)?);
                }

                models
            }
        },
    };
    let model = match parsed_args.arg_flags.get("--model") {
        Some(model) => get_model_by_name(&models, model)?,
        None => match &index {
            Ok(Ok(index)) => get_model_by_name(&models, &index.api_config.model)?,
            _ => match Index::load_config_from_home::<ApiConfig>("api.json") {
                Ok(Some(api_config)) => match api_config.model {
                    model => get_model_by_name(&models, &model)?,
                    _ => return Err(Error::ModelNotSelected),
                },
                _ => return Err(Error::ModelNotSelected),
            },
        },
    };
    let context = match parsed_args.arg_flags.get("--context") {
        Some(path) => {
            let s = fs_read_string(path)?;
            serde_json::from_str::<Value>(&s)?
        }
        None => Value::Object(Map::new()),
    };
    let (dump_pdl_at, dump_json_at) = match parsed_args.arg_flags.get("--log") {
        Some(log_at) => {
            let now = Local::now();

            if !exists(&str_to_pathbuf(log_at)) {
                create_dir(log_at)?;
            }

            (
                Some(str_to_pathbuf(&fs_join(
                    log_at,
                    &format!("{}.pdl", now.to_rfc3339()),
                )?)),
                Some(log_at.to_string()),
            )
        }
        None => (None, None),
    };
    let arg_schema = match parsed_args.arg_flags.get("--schema") {
        Some(schema) => Some(parse_schema(schema)?),
        None => None,
    };
    let pdl = parse_pdl_from_file(&pdl_at, &tera::Context::from_value(context)?, strict_mode)?;
    let schema = match (pdl.schema, arg_schema) {
        (_, Some(schema)) => Some(schema),
        (Some(schema), _) => Some(schema),
        _ => None,
    };
    let dump_api_usage_at = match &index {
        Ok(Ok(index)) => index.api_config.dump_api_usage_at(&index.root_dir, "pdl"),
        _ => None,
    };

    let request = Request::ChatRequest {
        messages: pdl.messages,
        schema: schema.clone(),
        model: model.clone(),
        dump_pdl_at: dump_pdl_at.map(|p| p.to_string_lossy().to_string()),
        dump_json_at,
        dump_api_usage_at,
        temperature: None,
        timeout: Some(model.api_timeout * 1_000),
        max_retry: 3,
        max_tokens: None,
        sleep_between_retries: 10_000,
        frequency_penalty: None,
        schema_max_try: 3,
    };

    match schema {
        Some(schema) => {
            let result = request
                .send_and_validate::<serde_json::Value>(serde_json::Value::Null)
                .await?;
            render_pdl_schema(&schema, &result)?;
        }
        None => {
            let response_str = request.send().await?.get_message(0).unwrap().to_string();
            println!("{response_str}");
        }
    }

    Ok(())
}