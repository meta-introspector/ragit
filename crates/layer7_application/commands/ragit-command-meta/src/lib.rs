use ragit_utils::prelude::*;
//use ragit_cli::prelude::*;
//use ragit_index_io::index_struct::load_index_from_path;
use ragit_index_types::index_struct::Index;
use ragit_index_core::load_index_from_path;

//use ragit_index_io::index_struct::{Index, load_index_from_path};
use ragit_utils::project_root::find_root;
use ragit_utils::cli_types::{CliError, Span};
use serde_json::Value;

pub fn meta_command_main(args: &[String]) -> Result<(), anyhow::Error> {
    let mut index = load_index_from_path(&find_root()?)?;

    match args.get(2).map(|s| s.as_str()) {
        Some("--get") => {
            let parsed_args = ArgParser::new()
                .optional_flag(&["--json"])
                .short_flag(&["--json"])
                .args(ArgType::String, ArgCount::Exact(1))
                .parse(args, 3)?;

            let args = parsed_args.get_args_exact(1)?;
            let key = args[0].to_string();
            let json_mode = parsed_args.get_flag(0).is_some();

	    // get_meta_by_key defined in ragit/crates/server/src/methods/repo_fs.rs
            let value = index.get_meta_by_key(key)?;
            if json_mode {
                println!("{value:?}");
            } else {
                println!("{:?}", value);
            }
        }
        Some("--get-all") => {
            let parsed_args = ArgParser::new()
                .optional_flag(&["--json"])
                .short_flag(&["--json"])
                .parse(args, 3)?;

            let json_mode = parsed_args.get_flag(0).is_some();
            let all = index.get_all_meta()?;

            if json_mode {
                println!("{}", serde_json::to_string_pretty(&all)?);
            } else {
                for (k, v) in all.iter() {
                    println!("{k}: {v}");
                }
            }
        }
        Some("--set" | "--add") => {
            let parsed_args = ArgParser::new()
                .args(ArgType::String, ArgCount::Exact(2))
                .parse(args, 3)?;
            let key_value = parsed_args.get_args_exact(2)?;
            let (key, value) = (key_value[0].to_string(), key_value[1].to_string());
            let prev_value = index.get_meta_by_key(key.clone())?;
            index.set_meta_by_key(key.clone(), value.clone())?;
            let new_value = index.get_meta_by_key(key.clone())?.unwrap();

            if let Some(prev_value) = prev_value {
                println!("metadata set `{key}`: `{prev_value}` -> `{new_value}`");
            } else {
                println!("metadata set `{key}`: `{new_value}`");
            }
        }
        Some("--remove" | "--unset") => {
            let parsed_args = ArgParser::new()
                .args(ArgType::String, ArgCount::Exact(1))
                .parse(args, 3)?;
            let key = &parsed_args.get_args_exact(1)?[0];
            let prev_value = index.remove_meta_by_key(key.to_string())?;

            println!("metadata unset `{key}`: {:?}", prev_value);
        }
        Some("--remove-all" | "--unset-all") => {
            ArgParser::new().parse(args, 3)?;

            index.remove_all_meta()?;
            println!("metadata removed");
        }
        Some(flag) => {
            return Err(anyhow::anyhow!(CliError::new_message_with_span(format!("Unknown flag: `{flag}`. Valid flags are --get | --get-all | --set | --remove | --remove-all."), Span::Exact(2).render(args, 2))));
        }
        None => {
            return Err(anyhow::anyhow!(CliError::new_message_with_span(
                String::from(
                    "Flag `--get | --get-all | --set | --remove | --remove-all` is missing.",
                ),
                Span::End.render(args, 2),
            )));
        }
    }

    Ok(())
}
