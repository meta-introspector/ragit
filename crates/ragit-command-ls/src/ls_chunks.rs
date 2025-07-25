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
use ragit_types::chunk::chunk_struct::Chunk;
use ragit_error::ApiError;

pub async fn ls_chunks_command_main(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .optional_flag(&["--uid-only", "--stat-only"])
        .optional_flag(&["--json"])
        .short_flag(&["--json"])
        .args(ArgType::UidOrPath, ArgCount::Any)
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/ls-chunks.txt"));
        return Ok(());
    }

    let uid_only = parsed_args.get_flag(0).unwrap_or_default() == "--uid-only";
    let stat_only = parsed_args.get_flag(0).unwrap_or_default() == "--stat-only";
    let json_mode = parsed_args.get_flag(1).is_some();
    let index = Index::load(find_root()?.into(), LoadMode::OnlyJson)?;
    let args = parsed_args.get_args();
    let chunks = if args.is_empty() {
        if !uid_only {
            if !json_mode {
                println!("{} chunks", index.chunk_count);
            } else if stat_only {
                println!("{{\"chunks\":{}}}", index.chunk_count);
            }
        }
        if stat_only {
            return Ok(());
        }
        index.list_chunks(
            &|_| true,  // no filter
            &|c| c,  // no map
            &|chunk| chunk.source.sortable_string(),  // sort by source
        )?
    } else {
        let query = uid_query(&index, &args, UidQueryConfig::new().file_or_chunk_only())?;
        let mut chunks = vec![];
        for uid in query.get_chunk_uids() {
            let chunk = index.get_chunk_by_uid(uid)?;
            chunks.push(chunk);
        }
        if chunks.is_empty() {
            for file_uid in query.get_file_uids() {
                let uids = index.get_chunks_of_file(file_uid)?;
                for uid in uids.iter() {
                    let chunk = index.get_chunk_by_uid(*uid)?;
                    chunks.push(chunk);
                }
            }
        }
        if chunks.is_empty() {
            return Err(Error::CliError(CliError::new_message(format!("There's no chunk/file that matches `{}`.", args.join(" ")))));
        }
        chunks
    };

    if json_mode {
        if uid_only {
            println!("{}", serde_json::to_string_pretty(&chunks.iter().map(|c| c.uid).collect::<Vec<_>>())?);
        } else {
            println!("{}", serde_json::to_string_pretty(&chunks)?);
        }
    } else if uid_only {
        for chunk in chunks {
            println!("{}", chunk.uid);
        }
    }

    Ok(())
}