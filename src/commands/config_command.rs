use crate::{Error, Index, LoadMode};
use ragit_cli::{ArgCount, ArgParser, ArgType, Span};
use serde_json::Value;

pub fn config_command(args: &[String]) -> Result<(), Error> {
    let mut index = Index::load(crate::find_root()?.to_string_lossy().into_owned(), LoadMode::OnlyJson)?;

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
                let models = Index::list_models(
                    &index.get_path().join("models.json"),
                    &|_| true,  // no filter
                    &|model| model,  // no map
                    &|model| model.name.to_string(),
                )?;

                if let Err(e @ ragit_api::Error::InvalidModelName { .. }) = ragit_api::get_model_by_name(&models, &value) {
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
                    println!(
                        "    {k:?}: {v}{}",
                        if i != kv.len() - 1 { "," } else { "" },
                    );
                }

                println!("}}");
            } else {
                for (k, v) in kv.iter() {
                    println!("{k}: {v}");
                }
            }
        }
        Some(flag) => {
            return Err(Error::CliError {
                message: format!("Unknown flag: `{flag}`. Valid flags are --get | --get-all | --set."),
                span: Span::Exact(2).render(args, 2).unwrap_rendered(),
            });
        }
        None => {
            return Err(Error::CliError {
                message: String::from("Flag `--get | --get-all | --set` is missing."),
                span: Span::End.render(args, 2).unwrap_rendered(),
            });
        }
    }

    Ok(())
}
