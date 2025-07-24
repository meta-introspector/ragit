use crate::prelude::*;

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
        match Index::load_config_from_home::<Value>("api.json") {
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
        None => Value::Object(serde_json::Map::new()),
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
        Some(ragit_api_prelude::parse_schema(schema)?)
    } else {
        None
    };

    let pdl_at = parsed_args.get_args_exact(1)?[0].clone();

    let strict_mode = true;

    ragit_api_prelude::parse_pdl_from_file(&pdl_at, &tera::Context::from_value(context)?, strict_mode)?;

    if let Some(schema) = schema {
        let result = ragit_api_prelude::render_pdl_schema(&schema, &Value::Null)?;
        println!("{}", result);
    }

    Ok(())
}