use crate::prelude::*;
use crate::main::main_find_root::find_root;
use ragit_utils::index::index_struct::Index;
use ragit_utils::index::load_mode::LoadMode;
use ragit_utils::index::commands::add::AddMode;
use ragit_utils::cli_types::{ArgParser, ArgType, ArgCount, Span};
use crate::commands::add_command::add_command;

pub async fn add_command_main(args: Vec<String>, _pre_args: ParsedArgs) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .optional_flag(&["--reject", "--force"])
        .optional_flag(&["--all"])
        .optional_flag(&["--dry-run"])
        .short_flag(&["--force"])
        .args(ArgType::Path, ArgCount::Any)
        .parse(&args, 2)?;

    if parsed_args.show_help() {
        println!("{}", include_str!("../../../docs/commands/add.txt"));
        return Ok(());
    }

    let root_dir = find_root()?;
    let mut index = Index::load(root_dir.to_string_lossy().into_owned().into(), LoadMode::QuickCheck)?;
    let add_mode = if parsed_args.get_flag(0).is_some() { Some(AddMode::Manual) } else { Some(AddMode::Auto) };
    let all = parsed_args.get_flag(1).is_some();
    let dry_run = parsed_args.get_flag(2).is_some();

    let mut files = parsed_args.get_args();

    if all {
        if !files.is_empty() {
            return Err(Error::CliError(ragit_utils::error::CliError::new_message("You cannot use `--all` option with paths.")));
        }

        files.push(root_dir.to_string_lossy().into_owned());
    }

    else if files.is_empty() {
        return Err(Error::CliError(ragit_utils::error::CliError::new_message_with_span("Please specify which files to add.", Span::End.render(&args, 2))));
    }

    let result = index.add_files_command(
        &files,
        add_mode,
        dry_run,
    ).await?;

    println!("{result}");
    Ok(())
}
