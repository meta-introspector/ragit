use ragit_utils::cli_types::ParsedArgs;
pub async fn archive_command_main(args: Vec<String>, _pre_args: ParsedArgs) -> Result<(), anyhow::Error> {
    panic!("FIXME: ragit-command-archive is commented out.");
}

/*
use ragit_utils::prelude::*;
use ragit_api::prelude::*;
use ragit_types::prelude::*;
use ragit_index_core::load_index_from_path;
use ragit_index_types::index_struct::Index;

use ragit_utils::project_root::find_root;
use ragit_utils::doc_utils::get_doc_content;
use ragit_utils::cli_types::CliError;

pub async fn archive_command_main(args: Vec<String>, _pre_args: ParsedArgs) -> Result<(), anyhow::Error> {
    let command = args.get(2).map(|arg| arg.as_str());

    match command {
        Some("create") => {
            let _index = load_index_from_path(
                &find_root()?.to_string_lossy().into_owned().into(),
            )?;
            let parsed_args = ArgParser::new()
                .arg_flag_with_default(
                    "--jobs",
                    "4",
                    ArgType::IntegerBetween {
                        min: Some(0),
                        max: None,
                    },
                )
                .optional_arg_flag(
                    "--size-limit",
                    ArgType::IntegerBetween {
                        min: Some(0),
                        max: None,
                    },
                )
                .arg_flag("--output", ArgType::Path)
                .flag_with_default(&["--no-configs", "--configs"])
                .flag_with_default(&["--no-prompts", "--prompts"])
                .optional_flag(&["--force"])
                .optional_flag(&["--quiet"])
                .short_flag(&["--force", "--output", "--quiet"])
                .parse(&args, 2)?;

            if parsed_args.show_help() {
                println!("{}", get_doc_content("commands/archive-create.txt"));
                return Ok(());
            }

            let _jobs = parsed_args
                .arg_flags
                .get("--jobs")
                .as_ref()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let _size_limit = parsed_args
                .arg_flags
                .get("--size-limit")
                .as_ref()
                .map(|n| n.parse::<u64>().unwrap());
            let _output = parsed_args
                .arg_flags
                .get("--output")
                .as_ref()
                .unwrap()
                .to_string();
            let _include_configs = parsed_args.get_flag(0).unwrap() == "--configs";
            let _include_prompts = parsed_args.get_flag(1).unwrap() == "--prompts";
            let _force = parsed_args.get_flag(2).is_some();
            let _quiet = parsed_args.get_flag(3).is_some();
        }
        Some("extract") => {
            // TODO: implement extract
            println!("extract command is not implemented yet");
        }
        _ => {
            return Err(anyhow::anyhow!(CliError::new_message(
                "Unknown archive command.".to_string(),
            )));
        }
    }
    Ok(())
}
*/