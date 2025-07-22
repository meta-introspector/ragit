use crate::prelude::*;
use ragit_cli::{ArgCount, ArgParser, ArgType, Span};

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

    let root_dir = crate::find_root()?;
    let mut index = Index::load(root_dir.to_string_lossy().into_owned(), LoadMode::QuickCheck)?;
    let add_mode = parsed_args.get_flag(0).map(|flag| AddMode::parse_flag(&flag)).unwrap_or(None);
    let all = parsed_args.get_flag(1).is_some();
    let dry_run = parsed_args.get_flag(2).is_some();
    //    let ignore_file = index.read_ignore_file_command(&root_dir.to_string_lossy().into_owned())?;

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

    // if it's `--reject` mode, it first runs with `--dry-run` mode.
    // if the dry_run has no problem, then it actually runs
    let result = index.add_files_command(
        &files,
        add_mode,
        dry_run || add_mode == Some(AddMode::Reject),
        // &ignore_file,
    ).await?;

    if add_mode == Some(AddMode::Reject) && !dry_run {
        index.add_files_command(&files, add_mode, dry_run, /* &ignore_file */).await?;
    }

    println!("{result}");
    Ok(())
}
