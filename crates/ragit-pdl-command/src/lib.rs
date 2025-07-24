use crate::prelude::*;
use ragit_pdl::parse_schema;
use std::result::Result::Ok;
use ragit_index::Index;
use ragit_api::{ApiError, Model, ModelRaw, get_model_by_name, render_pdl_schema};
use ragit_utils::prelude::{ArgParser, ArgType, ArgCount, read_string, join};
use serde_json::Value;
use chrono::{Local, Datelike, Timelike};

pub async fn pdl_command(root_dir: PathBuf, args: &[String]) -> Result<(), ApiError> {
    let parsed_args = ArgParser::new()
        .optional_arg_flag("--model", ArgType::String)
        .optional_arg_flag("--models", ArgType::Path)
        .optional_arg_flag("--context", ArgType::Path)
        .optional_arg_flag("--log", ArgType::Path)
        .optional_arg_flag("--schema", ArgType::String)
        .args(ArgType::Path, ArgCount::Exact(1))
        .parse(args, 2)?;

    let index = Index::load(root_dir.clone(), LoadMode::OnlyJson)?;

    let models = if let Some(models_at) = parsed_args.get_optional_arg_flag("--models") {
        let m = read_string(&models_at)?;
        let models_raw = serde_json::from_str::<Vec<ModelRaw>>(&m)?;
        let mut models = Vec::with_capacity(models_raw.len());

        for model_raw in models_raw.iter() {
            models.push(Model::try_from(model_raw)?);
        }

        models
    } else {
        let models_raw = ModelRaw::default_models();
        let mut models = Vec::with_capacity(models_raw.len());

        for model_raw in models_raw.iter() {
            models.push(Model::try_from(model_raw)?);
        }

        models
    };

    let model = if let Some(model) = parsed_args.get_optional_arg_flag("--model") {
        get_model_by_name(&models, model)?
    } else {
        match Index::load_config_from_home_dir::<Value>("api.json") {
            Ok(Some(Value::Object(api_config))) => match api_config.get("model") {
                Some(Value::String(model)) => get_model_by_name(&models, model)?,
                _ => return Err(ApiError::ModelNotSelected),
            },
            _ => return Err(ApiError::ModelNotSelected),
        }
    };

    let context = if let Some(context_at) = parsed_args.get_optional_arg_flag("--context") {
        let s = read_string(&context_at)?;
        serde_json::from_str::<Value>(&s)?
    } else {
        serde_json::Value::Null
    };

    let log_at = if let Some(log_at) = parsed_args.get_optional_arg_flag("--log") {
        let now = Local::now();
        let log_at = join(
            &log_at,
            &format!(
                "{}-{}-{}-{}-{}-{}.log",
                now.year(),
                now.month(),
                now.day(),
                now.hour(),
                now.minute(),
                now.second()
            ),
        )?;
        Some(log_at)
    } else {
        None
    };

    let schema = if let Some(schema) = parsed_args.get_optional_arg_flag("--schema") {
        Some(parse_schema(schema)?)
    } else {
        None
    };

    let pdl_at = parsed_args.get_args_exact(1)?[0].clone();

    let strict_mode = true;

    ragit_pdl::parse_pdl_from_file(&pdl_at, &tera::Context::from_value(context)?, strict_mode)?;

    if let Some(schema) = schema {
        let result = render_pdl_schema(&schema, &Value::Null)?;
        println!("{}", result);
    }

    Ok(())
}
