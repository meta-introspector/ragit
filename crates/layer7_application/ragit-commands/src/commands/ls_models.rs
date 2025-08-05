use crate::prelude::*;
use ragit_utils::cli_types::{ArgParser, ArgType, ArgCount};
use ragit_utils::doc_utils::get_doc_content;

pub async fn ls_models_command_main(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .optional_flag(&["--name-only", "--stat-only"])
        .optional_flag(&["--selected"])
        .optional_flag(&["--json"])
        .short_flag(&["--json"])
        .alias("--cached", "--staged")
        .args(ArgType::String, ArgCount::Leq(1))
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/ls-models.txt"));
        return Ok(());
    }

    let name_only = parsed_args.get_flag(0).unwrap_or_default() == "--name-only";
    let stat_only = parsed_args.get_flag(0).unwrap_or_default() == "--stat-only";
    let selected_only = parsed_args.get_flag(1).is_some();
    let json_mode = parsed_args.get_flag(2).is_some();
    let args = parsed_args.get_args();
    let index = Index::load(find_root()?.into(), LoadMode::OnlyJson)?;
    let mut models = list_models(
        &find_root()?.join("models.json"),
        &|_| true,  // no filter
        &|model| model,  // no map
        &|model| model.name.to_string(),
    )?;
    if selected_only {
        if !args.is_empty() {
            return Err(Error::CliError(CliError::new_message("You cannot use `--selected` option with a model name.".to_string())));
        }
        models = match get_model_by_name(&models, &index.api_config.model) {
            Ok(model) => vec![model.clone()],
            Err(_) => {
                // models = match index.find_lowest_cost_model() {
                //     Some(model) => vec![model.clone()],
                //     None => vec![],
                // };
                vec![]
            },
        };
    } else if let Some(model) = args.get(0) {
        models = match get_model_by_name(&models, model) {
            Ok(model) => vec![model.clone()],
            Err(ApiError::InvalidModelName { candidates, .. }) => {
                models.into_iter().filter(|model| candidates.contains(&model.name)).collect()
            }
            Err(_) => vec![],
        };
    }
    if !json_mode && !name_only {
        println!("{} models", models.len());
    }

    if stat_only {
        return Ok(());
    }

    if json_mode {
        if name_only {
            println!("{}", serde_json::to_string_pretty(&models.iter().map(|model| model.name.clone()).collect::<Vec<_>>())?);
        } else {
            println!("{}", serde_json::to_string_pretty(&models)?);
        }
    } else if name_only {
        for model in models {
            println!("{}", model.name);
        }
    } else {
        for model in models {
            println!("----------");
            println!("name: {}", model.name);
            // println!("provider: {}", model.provider);
            // println!("cost: {}", model.cost);
            // println!("max_input_tokens: {}", model.max_input_tokens);
            // println!("max_output_tokens: {}", model.max_output_tokens);
            // println!("quality: {}", model.quality);
        }
    }

    Ok(())
}