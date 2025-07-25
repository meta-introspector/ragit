use ragit_utils::prelude::*;
use ragit_api::prelude::*;
use ragit_types::prelude::*;
use serde_json::Value;
use serde_json::Map;

pub async fn model_command_main(args: &[String]) -> Result<(), Error> {
    match args.get(2).map(|s| s.as_str()) {
        Some("--search") => {
            let parsed_args = ArgParser::new()
                .arg_flag_with_default("--remote", "https://ragit.baehyunsol.com", ArgType::Path)
                .optional_flag(&["--name-only", "--stat-only"])
                .optional_flag(&["--json"])
                .short_flag(&["--json"])
                .args(ArgType::String, ArgCount::Exact(1))
                .parse(args, 3)?;

            let name_only = parsed_args.get_flag(0).unwrap_or_default() == "--name-only";
            let stat_only = parsed_args.get_flag(1).unwrap_or_default() == "--stat-only";
            let json_mode = parsed_args.get_flag(2).is_some();
            let remote = parsed_args.arg_flags.get("--remote").unwrap();
            let keyword = parsed_args.get_args_exact(1)?[0].to_string();
            let models = Index::search_remote_models(&keyword, remote).await?;

            if !json_mode && !name_only {
                println!("{} models", models.len());
            }

            if stat_only {
                if json_mode {
                    println!("{{\"models\":{}}}", models.len());
                }

                return Ok(());
            }

            if json_mode {
                if name_only {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(
                            &models.iter().map(|model| &model.name).collect::<Vec<_>>()
                        )?
                    );
                } else {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(
                            &models
                                .iter()
                                .map(|model| {
                                    vec![
                                        (String::from("name"), model.name.clone().into()),
                                        (
                                            String::from("api_provider"),
                                            model.api_provider.to_string().into(),
                                        ),
                                        (
                                            String::from("api_key_env_var"),
                                            model.api_env_var.clone().into(),
                                        ),
                                        (
                                            String::from("can_read_images"),
                                            model.can_read_images.into(),
                                        ),
                                        (
                                            String::from("dollars_per_1b_input_tokens"),
                                            model.dollars_per_1b_input_tokens.into(),
                                        ),
                                        (
                                            String::from("dollars_per_1b_output_tokens"),
                                            model.dollars_per_1b_output_tokens.into(),
                                        ),
                                    ]
                                    .into_iter()
                                    .collect::<Map<String, Value>>()
                                })
                                .collect::<Vec<_>>(),
                        )?,
                    );
                }
            } else {
                for model in models.iter() {
                    if name_only {
                        println!("{}", model.name);
                        continue;
                    }

                    println!("----------");
                    println!("name: {}", model.name);
                    println!("provider: {}", model.api_provider);

                    if let Some(api_env_var) = &model.api_env_var {
                        println!("api_key_env_var: {api_env_var}");
                    }

                    println!("can_read_images: {}", model.can_read_images);
                    println!(
                        "dollars_per_1b_input_tokens: {}",
                        model.dollars_per_1b_input_tokens
                    );
                    println!(
                        "dollars_per_1b_output_tokens: {}",
                        model.dollars_per_1b_output_tokens
                    );
                }
            }
        }
        Some("--fetch") => {
            let parsed_args = ArgParser::new()
                .arg_flag_with_default("--remote", "https://ragit.baehyunsol.com", ArgType::Path)
                .optional_flag(&["--all"])
                .optional_flag(&["--existing-only"])
                .optional_flag(&["--quiet"])
                .short_flag(&["--all", "--quiet"])
                .args(ArgType::String, ArgCount::Leq(1))
                .parse(args, 3)?;

            let mut index = Index::load(find_root()?.into(), LoadMode::OnlyJson)?;
            let all = parsed_args.get_flag(0).is_some();
            let existing_only = parsed_args.get_flag(1).is_some();
            let quiet = parsed_args.get_flag(2).is_some();
            let model_name = parsed_args.get_args().get(0).map(|model| model.to_string());
            let remote = parsed_args.arg_flags.get("--remote").unwrap();

            let result = if let Some(model_name) = model_name {
                if all {
                    return Err(Error::CliError(CliError::new_message(
                        "You cannot use `--all` option with a model name.".to_string(),
                    )));
                }

                index
                    .fetch_remote_models(&model_name, existing_only, remote)
                    .await?
            } else if all {
                index.fetch_all_remote_models(existing_only, remote).await?
            } else {
                return Err(Error::CliError(CliError::new_message_with_span(
                    "Please specify which model to fetch.".to_string(),
                    Span::End.render(args, 2),
                )));
            };

            if !quiet {
                println!(
                    "fetched {} new models, updated {} models",
                    result.fetched, result.updated
                );
            }
        }
        Some("--remove") => {
            let parsed_args = ArgParser::new()
                .optional_flag(&["--all"])
                .short_flag(&["--all"])
                .args(ArgType::String, ArgCount::Leq(1))
                .parse(args, 3)?;

            let mut index = Index::load(find_root()?.into(), LoadMode::OnlyJson)?;
            let all = parsed_args.get_flag(0).is_some();
            let model_name = parsed_args.get_args().get(0).map(|model| model.to_string());

            if let Some(model_name) = model_name {
                if all {
                    return Err(Error::CliError(CliError::new_message(
                        "You cannot use `--all` option with a model name.".to_string(),
                    )));
                }

                index.remove_local_model(&model_name)?;
            } else if all {
                index.remove_all_local_models()?;
            }
        }
        Some(flag) => {
            return Err(Error::CliError(CliError::new_message_with_span(
                format!("Unknown flag: `{flag}`. Valid flags are --search | --update | --remove."),
                Span::End,
            )));
        }
        None => {
            return Err(Error::CliError(CliError::new_message_with_span(
                String::from("Flag `--search | --update | --remove` is missing."),
                Span::End,
            )));
        }
    }

    Ok(())
}
