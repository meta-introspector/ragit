use ragit_utils::cli_types::{ArgParser, ArgType, ArgCount, Span};
use ragit_utils::doc_utils::get_doc_content;
//use ragit_index_io::load_index_from_path;
//use ragit_index_core::Index;
use ragit_utils::project_root::find_root;
use ragit_utils::error::{CliError};
use std::path::PathBuf;
use serde_json::Value;
use std::collections::HashMap;
use ragit_api::list_models;
use ragit_api::get_model_by_name;
//use ragit_api::ApiError;
use ragit_index_core::load_index_from_path;
use ragit_types::ApiError;
pub async fn run(args: &[String]) -> Result<(), anyhow::Error> {
    let mut index = load_index_from_path(&find_root()?)?;

    match args.get(2).map(|s| s.as_str()) {
        Some("--get") => {
            let parsed_args = ArgParser::new()
                .optional_flag(&["--json"])
                .short_flag(&["--json"])
                .args(ArgType::String, ArgCount::Exact(1))
                .parse(args, 3)?;

            let args = parsed_args.get_args_exact(1)?;
            let json_mode = parsed_args.get_flag(0).is_some();

            let s = match index.get_config_by_key(args[0].clone())? {
                Value::String(s) => {
                    if json_mode {
                        format!("{s:?}")
                    } else {
                        s.to_string()
                    }
                }
                v => v.to_string(),
            };
            println!("{s}");
        }
        Some("--set") => {
            let parsed_args = ArgParser::new()
                .args(ArgType::String, ArgCount::Exact(2))
                .parse(args, 3)?;
            let args = parsed_args.get_args_exact(2)?;
            let key = args[0].clone();
            let value = args[1].clone();

            // QoL improvement: it warns if the user typed a wrong model name.
            if &key == "model" {
                let models = list_models(
                    &index.get_path().join("models.json").to_string_lossy(),
                    &|_| true,  // no filter
                    &|model| model, // no map
                    &|model| model.name.to_string(),
                )?;

                if let Err(e @ ApiError::InvalidModelName { .. }) =
                    get_model_by_name(&models, &value)
                {
                    return Err(e.into());
                }
            }

            let previous_value = index.set_config_by_key(key.clone(), value.clone())?;

            match previous_value {
                Some(prev) => {
                    println!("set `{key}`: `{prev}` -> `{value}`");
                }
                None => {
                    println!("set `{key}`: `{value}`");
                }
            }
        }
        Some("--get-all") => {
            let parsed_args = ArgParser::new()
                .optional_flag(&["--json"])
                .short_flag(&["--json"])
                .parse(args, 3)?;

            let json_mode = parsed_args.get_flag(0).is_some();
            let mut kv = index.get_all_configs()?;
            kv.sort_by_key(|(k, _)| k.to_string());

            if json_mode {
                println!("{{");

                for (i, (k, v)) in kv.iter().enumerate() {
                    println!("    {k:?}: {v}{}", if i != kv.len() - 1 { "," } else { "" },);
                }

                println!("}}");
            } else {
                for (k, v) in kv.iter() {
                    println!("{k}: {v}");
                }
            }
        }
        Some(flag) => {
            return Err(anyhow::anyhow!(CliError::new_message_with_span(
                format!("Unknown flag: `{flag}`. Valid flags are --get | --get-all | --set."),
                Span::End.render(args, 2),
            )));
        }
        None => {
            return Err(anyhow::anyhow!(CliError::new_message_with_span(
                String::from("Flag `--get | --get-all | --set` is missing."),
                Span::End.render(args, 2),
            )));
        }
    }

    Ok(())
}
