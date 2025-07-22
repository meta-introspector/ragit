use crate::prelude::*;
use crate::main::main_find_root::find_root;
use ragit_utils::index::index_struct::Index;
use ragit_utils::index::load_mode::LoadMode;
use ragit_args::{ArgParser, ArgType, ArgCount};
use ragit_utils::index::commands::archive::create::archive_create_command;
use ragit_utils::index::commands::archive::extract::archive_extract_command;

pub async fn archive_command_main(args: Vec<String>, pre_args: ragit_args::ParsedArgs) -> Result<(), Error> {
    let command = args.get(2).map(|arg| arg.as_str());

    match command {
        Some("create") => {
            let index = Index::load(find_root()?.to_string_lossy().into_owned().into(), LoadMode::QuickCheck)?;
            let parsed_args = ArgParser::new()
                .arg_flag_with_default("--jobs", "4", ArgType::IntegerBetween { min: Some(0), max: None })
                .optional_arg_flag("--size-limit", ArgType::IntegerBetween { min: Some(0), max: None })
                .arg_flag("--output", ArgType::Path)
                .flag_with_default(&["--no-configs", "--configs"])
                .flag_with_default(&["--no-prompts", "--prompts"])
                .optional_flag(&["--force"])
                .optional_flag(&["--quiet"])
                .short_flag(&["--force", "--output", "--quiet"])
                .parse(&args, 2)?;

            if parsed_args.show_help() {
                println!("{}", include_str!("../../../docs/commands/archive-create.txt"));
                return Ok(());
            }

            let jobs = parsed_args.arg_flags.get("--jobs").as_ref().unwrap().parse::<usize>().unwrap();
            let size_limit = parsed_args.arg_flags.get("--size-limit").as_ref().map(|n| n.parse::<u64>().unwrap());
            let output = parsed_args.arg_flags.get("--output").as_ref().unwrap().to_string();
            let include_configs = parsed_args.get_flag(0).unwrap() == "--configs";
            let include_prompts = parsed_args.get_flag(1).unwrap() == "--prompts";
            let force = parsed_args.get_flag(2).is_some();
            let quiet = parsed_args.get_flag(3).is_some();
            
        }
        Some("extract") => {
            // TODO: implement extract
            println!("extract command is not implemented yet");
        }
        _ => {
            return Err(Error::CliError {
                message: String::from("Unknown archive command."),
                span: (String::new(), 0, 0),
            });
        }
    }
    Ok(())
}