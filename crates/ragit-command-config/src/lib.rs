use ragit_utils::prelude::*;
use ragit_api::prelude::*;
use ragit_types::prelude::*;

pub fn config_command_main(args: &[String]) -> Result<(), Error> {
    let mut index = Index::load(find_root()?.into(), LoadMode::OnlyJson)?;

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
                let models = ragit_api::list_models(
                    &index.get_path().join("models.json").to_string_lossy(),
                    &|_| true,      // no filter
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
            return Err(Error::CliError(CliError::new_message_with_span(
                format!("Unknown flag: `{flag}`. Valid flags are --get | --get-all | --set."),
                Span::End.render(args, 2),
            )));
        }
        None => {
            return Err(Error::CliError(CliError::new_message_with_span(
                String::from("Flag `--get | --get-all | --set` is missing."),
                Span::End.render(args, 2),
            )));
        }
    }

    Ok(())
}
