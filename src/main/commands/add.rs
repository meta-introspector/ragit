use crate::prelude::*;
use crate::main::find_root;
use crate::{AddMode, Index, LoadMode};

pub async fn add_command_main(args: Vec<String>, _pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
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
            return Err(Error::CliError {
                message: String::from("You cannot use `--all` option with paths."),
                span: (String::new(), 0, 0),  // TODO
            });
        }

        files.push(root_dir.to_string_lossy().into_owned());
    }

    else if files.is_empty() {
        return Err(Error::CliError {
            message: String::from("Please specify which files to add."),
            span: Span::End.render(&args, 2).unwrap_rendered(),
        });
    }

    let result = index.add_files_command(
        &files,
        add_mode,
        dry_run,
    ).await?;

    println!("{result}");
    Ok(())
}
