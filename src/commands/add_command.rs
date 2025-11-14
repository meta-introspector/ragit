use ragit_commands::{AddMode, Error, Index, LoadMode, ArgCount, ArgParser, ArgType, Span, find_root, CliError};
use std::path::PathBuf;

pub async fn add_command(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .optional_flag(&["--reject", "--force"])
        .optional_flag(&["--all"])
        .optional_flag(&["--dry-run"])
        .short_flag(&["--force"])
        .args(ArgType::Path, ArgCount::Any)
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", include_str!("../../docs/commands/add.txt"));
        return Ok(());
    }

    let root_dir = find_root()?;
    let mut index = Index::load(root_dir.into(), LoadMode::QuickCheck)?;
    let add_mode = parsed_args.get_flag(0).map(|flag| AddMode::parse_flag(&flag)).unwrap_or(None);
    let all = parsed_args.get_flag(1).is_some();
    let dry_run = parsed_args.get_flag(2).is_some();
    let ignore_file = index.read_ignore_file_command(&root_dir.to_string_lossy().into_owned())?;

    let mut files = parsed_args.get_args();

    if all {
        if !files.is_empty() {
            return Err(Error::CliError::new("You cannot use `--all` option with paths.", Span::End));
        }

        files.push(root_dir.to_string_lossy().into_owned());
    }

    else if files.is_empty() {
        return Err(Error::CliError::new("Please specify which files to add.", Span::End.render(args, 2).unwrap_rendered()));
    }

    // if it's `--reject` mode, it first runs with `--dry-run` mode.
    // if the dry_run has no problem, then it actually runs
    let result = index.add_files_command(
        &files,
        add_mode,
        dry_run || add_mode == Some(AddMode::Reject),
        &ignore_file,
    ).await?;

    if add_mode == Some(AddMode::Reject) && !dry_run {
        index.add_files_command(&files, add_mode, dry_run).await?;
    }

    println!("{result}");
    Ok(())
}
