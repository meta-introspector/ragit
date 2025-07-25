use crate::prelude::*;
use ragit_utils::cli_types::{ArgParser, ArgType, ArgCount};
use ragit_utils::doc_utils::get_doc_content;
use ragit_index_io::index_struct::Index;
use ragit_index::LoadMode;
use ragit_utils::project_root::find_root;
use ragit_utils::uid::uid_query;
use ragit_utils::uid::UidQueryConfig;
use ragit_utils::error::{Error, CliError};
use std::path::PathBuf;
use serde_json::Value;
use std::collections::HashMap;
use ragit_types::uid::Uid;
use ragit_types::file_schema::FileSchema;
use ragit_schema::get_file_schema;

pub async fn ls_files_command_main(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .optional_flag(&["--name-only", "--uid-only", "--stat-only"])
        .optional_flag(&["--staged", "--processed"])
        .optional_flag(&["--json"])
        .short_flag(&["--json"])
        .alias("--cached", "--staged")
        .args(ArgType::UidOrPath, ArgCount::Any)
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/ls-files.txt"));
        return Ok(());
    }

    let name_only = parsed_args.get_flag(0).unwrap_or_default() == "--name-only";
    let uid_only = parsed_args.get_flag(0).unwrap_or_default() == "--uid-only";
    let stat_only = parsed_args.get_flag(0).unwrap_or_default() == "--stat-only";
    let staged = parsed_args.get_flag(1).unwrap_or_else(|| "--staged".to_string()) == "--staged";
    let processed = parsed_args.get_flag(1).unwrap_or_else(|| "--processed".to_string()) == "--processed";
    let json_mode = parsed_args.get_flag(2).is_some();
    let index = Index::load(find_root()?.into(), LoadMode::OnlyJson)?;
    let args = parsed_args.get_args();

    let files = if args.is_empty() {
        if !uid_only && !name_only {
            let staged_files = if staged { index.staged_files.len() } else { 0 };
            let processed_files = if processed { index.processed_files.len() } else { 0 };

            if json_mode && stat_only {
                println!(
                    "{{\"total files\":{},\"staged files\":{},\"processed files\":{}}}",
                    staged_files + processed_files,
                    staged_files,
                    processed_files,
                );
            } else if !json_mode {
                println!(
                    "{} total files, {} staged files, {} processed files",
                    staged_files + processed_files,
                    staged_files,
                    processed_files,
                );
            }
        }

        if stat_only {
            return Ok(());
        }

        index.list_files(
            &|f| (staged && !f.is_processed) || (processed && f.is_processed),
            &|f| f,  // no map
            &|f| f.path.to_string(),
        )?
    } else {
        let query = uid_query(&index, &args, UidQueryConfig::new().file_only())?;
        let mut files = vec![];
        let mut processed_files_len = 0;
        let mut staged_files_len = 0;

        if processed {
            for (path, uid) in query.get_processed_files() {
                processed_files_len += 1;
                files.push(ragit_schema::get_file_schema(&index, Some(path), Some(uid))?);
            }
        }

        if staged {
            for path in query.get_staged_files() {
                staged_files_len += 1;
                files.push(ragit_schema::get_file_schema(&index, Some(path), None)?);
            }
        }

        if files.is_empty() {
            return Err(Error::CliError(CliError::new_message(format!("There's no file that matches `{}`.", args.join(" ")))));
        }

        if !uid_only && !name_only {
            if json_mode && stat_only {
                println!(
                    "{{\"total files\":{},\"staged files\":{},\"processed files\":{}}}",
                    staged_files_len + processed_files_len,
                    staged_files_len,
                    processed_files_len,
                );
            } else if !json_mode {
                println!(
                    "{} total files, {} staged files, {} processed files",
                    staged_files_len + processed_files_len,
                    staged_files_len,
                    processed_files_len,
                );
            }
        }

        if stat_only {
            return Ok(());
        }

        files
    };

    if json_mode {
        if name_only {
            println!(
                "{}",
                serde_json::to_string_pretty(&files.iter().map(|file| file.path.to_string()).collect::<Vec<_>>())?,
            );
        } else if uid_only {
            println!(
                "{}",
                serde_json::to_string_pretty(&files.iter().map(|file| file.uid.to_string()).collect::<Vec<_>>())?,
            );
        } else {
            println!(
                "{}",
                serde_json::to_string_pretty(&files.prettify()?)?,
            );
        }
    } else {
        for file in files.iter() {
            if name_only {
                println!("{}", file.path);
                continue;
            }

            println!("----------");
            println!("name: {}{}", file.path, if file.is_processed { "" } else { " (not processed yet)" });

            if file.is_processed {
                println!("length: {}", file.length);
                println!("uid: {}", file.uid);
                println!("chunks: {}", file.chunks);
            }
        }
    }

    Ok(())
}
