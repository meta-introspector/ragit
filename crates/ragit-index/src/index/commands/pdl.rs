use crate::prelude::*;
use crate::index::load_mode::LoadMode;
use ragit_api::{ModelRaw, Model, get_model_by_name, Request};
use chrono::Local;

pub async fn pdl_command(root_dir: PathBuf, args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .flag_with_default(&["--strict", "--no-strict"])
        .optional_arg_flag("--model", ArgType::String)
        .optional_arg_flag("--models", ArgType::Path)
        .optional_arg_flag("--context", ArgType::Path)
        .optional_arg_flag("--log", ArgType::Path)
        .optional_arg_flag("--schema", ArgType::String)
        .args(ArgType::Path, ArgCount::Exact(1))
        .parse(args, 2)?;

    let index = Index::load(root_dir.clone(), LoadMode::OnlyJson);
    let pdl_at = parsed_args.get_args_exact(1)?[0].clone();
    let strict_mode = parsed_args.get_flag(0).unwrap() == "--strict";
    let models = match parsed_args.arg_flags.get("--models") {
        Some(path) => {
            let m = read_string(path)?;
            let models_raw = serde_json::from_str::<Vec<ModelRaw>>(&m)?;
            let mut models = Vec::with_capacity(models_raw.len());

            for model_raw in models_raw.iter() {
                models.push(Model::try_from(model_raw)?);
            }

            models
        }
        None => match &index {
            Ok(index) => index.models.clone(),
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
            Ok(index) => get_model_by_name(&models, &index.api_config.model)?,
            _ => match Index::load_config_from_home::<Value>("api.json") {
                Ok(Some(Value::Object(api_config))) => match api_config.get("model") {
                    Some(Value::String(model)) => get_model_by_name(&models, model)?,
                    _ => return Err(Error::ModelNotSelected),
                },
                _ => return Err(Error::ModelNotSelected),
            },
        },
    };
    let context = match parsed_args.arg_flags.get("--context") {
        Some(path) => {
            let s = read_string(path)?;
            serde_json::from_str::<Value>(&s)?
        }
        None => Value::Object(serde_json::Map::new()),
    };
    let (dump_pdl_at, dump_json_at) = match parsed_args.arg_flags.get("--log") {
        Some(log_at) => {
            let now = Local::now();

            if !exists(&PathBuf::from(log_at)) {
                create_dir(
                    PathBuf::from(log_at)
                        .to_str()
                        .ok_or_else(|| Error::Internal(format!("Invalid path: {}", log_at)))?,
                )?;
            }

            (
                Some(join(
                    PathBuf::from(log_at)
                        .to_str()
                        .ok_or_else(|| Error::Internal(format!("Invalid path: {}", log_at)))?,
                    &format!("{}.pdl", now.to_rfc3339()),
                )?),
                Some(log_at.to_string()),
            )
        }
        None => (None, None),
    };
    let arg_schema = match parsed_args.arg_flags.get("--schema") {
        Some(schema) => Some(ragit_pdl::parse_schema(schema)?),
        None => None,
    };
    let pdl =
        ragit_pdl::parse_pdl_from_file(&pdl_at, &tera::Context::from_value(context)?, strict_mode)?;
    let schema = match (pdl.schema, arg_schema) {
        (_, Some(schema)) => Some(schema),
        (Some(schema), _) => Some(schema),
        _ => None,
    };
    let dump_api_usage_at = match &index {
        Ok(index) => index.api_config.dump_api_usage_at(&index.root_dir, "pdl"),
        _ => None,
    };

    let request = Request {
        messages: pdl.messages,
        schema: schema.clone(),
        model: model.clone(),
        dump_pdl_at,
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
