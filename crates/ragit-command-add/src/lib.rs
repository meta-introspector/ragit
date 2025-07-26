use ragit_utils::prelude::*;
use ragit_api::prelude::*;
use ragit_types::prelude::*;
use ragit_types::add_mode::AddMode;

use ragit_utils::project_root::find_root;
use ragit_utils::doc_utils::get_doc_content;
use ragit_utils::cli_types::{CliError, Span};
use ragit_index_core::load_index_from_path;
use std::path::PathBuf;

pub async fn add_command_main(args: &[String]) -> Result<(), anyhow::Error> {
    let parsed_args = ArgParser::new()
        .optional_flag(&["--reject", "--force"])
        .optional_flag(&["--all"])
        .optional_flag(&["--dry-run"])
        .short_flag(&["--force"])
        .args(ArgType::Path, ArgCount::Any)
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/add.txt"));
        return Ok(());
    }

    let root_dir = find_root()?;
    let mut index = load_index_from_path(
        &PathBuf::from(root_dir.to_string_lossy().into_owned()),
    )?;
    let add_mode = parsed_args
        .get_flag(0)
        .map(|flag| AddMode::parse_flag(&flag))
        .unwrap_or(None);
    let all = parsed_args.get_flag(1).is_some();
    let dry_run = parsed_args.get_flag(2).is_some();

    let mut files = parsed_args.get_args();

    if all {
        if !files.is_empty() {
            return Err(anyhow::anyhow!(CliError::new_message(
                "You cannot use `--all` option with paths.".to_string(),
            )));
        }

        files.push(root_dir.to_string_lossy().into_owned());
    } else if files.is_empty() {
        return Err(anyhow::anyhow!(CliError::new_message_with_span(
            "Please specify which files to add.".to_string(),
            Span::End.render(args, 2),
        )));
    }

    let result = index.add_files_command(
        files,
        add_mode.clone(),
        dry_run || add_mode.clone() == Some(AddMode::Reject),
    )
    .await?;

    if add_mode.clone() == Some(AddMode::Reject) && !dry_run {
        index.add_files_command(&files, add_mode, dry_run).await?;
    }

    println!("{result}");
    Ok(())
}
