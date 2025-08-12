use ragit_utils::prelude::*;
// use ragit_index_core::load_index_from_path;
// use ragit_index_core::index_struct::Index;
// use ragit_query::query_helpers::{uid_query, UidQueryConfig};

pub async fn ls_chunks_command_main(_args: &[String]) -> Result<(), anyhow::Error> {
    panic!("FIX ME LATER: Fix the bootstrap first and this code later.");
/*
use ragit_utils::cli_types::{ArgParser, ArgType, ArgCount, CliError};
use ragit_utils::doc_utils::get_doc_content;
use ragit_index_core::load_index_from_path;
use ragit_index_core::index_struct::Index;
use ragit_utils::project_root::find_root;
use ragit_query::query_helpers::{uid_query, UidQueryConfig};
use std::path::PathBuf;
use serde_json::Value;
use std::collections::HashMap;
use ragit_types::uid::Uid;
use ragit_types::chunk::chunk_struct::Chunk;
use ragit_types::ApiError;

pub async fn ls_chunks_command_main(args: &[String]) -> Result<(), anyhow::Error> {
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
    let index = load_index_from_path(&find_root()?)?;
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
            &|c| c.clone(),  // no map
            &|chunk| chunk.title.clone(),  // sort by title
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
            return Err(anyhow::anyhow!(CliError::new_message(format!("There's no chunk/file that matches `{}`.", args.join(" ")))));
        }
        chunks.into_iter().map(|c| c.into()).collect()
    };

    if json_mode {
        if uid_only {
            println!("{}", serde_json::to_string_pretty(&chunks.iter().map(|c| c.title.clone()).collect::<Vec<_>>())?);
        } else {
            println!("{}", serde_json::to_string_pretty(&chunks)?);
        }
    } else if uid_only {
        for chunk in chunks {
            println!("{}", chunk.title);
        }
    }

    Ok(())
}
*/
    // let parsed_args = ArgParser::new()
    //     .optional_flag(&["--uid-only", "--stat-only"])
    //     .optional_flag(&["--json"])
    //     .short_flag(&["--json"])
    //     .args(ArgType::UidOrPath, ArgCount::Any)
    //     .parse(args, 2)?;

    // if parsed_args.show_help() {
    //     println!("{}", get_doc_content("commands/ls-chunks.txt"));
    //     return Ok(());
    // }

    // let uid_only = parsed_args.get_flag(0).unwrap_or_default() == "--uid-only";
    // let stat_only = parsed_args.get_flag(0).unwrap_or_default() == "----stat-only";
    // let json_mode = parsed_args.get_flag(1).is_some();
    // let index = load_index_from_path(&find_root()?)?;
    // let args = parsed_args.get_args();
    // let chunks = if args.is_empty() {
    //     if !uid_only {
    //         if !json_mode {
    //             println!("{} chunks", index.chunk_count);
    //         } else if stat_only {
    //             println!("{{\"chunks\":{}}}", index.chunk_count);
    //         }
    //     }
    //     if stat_only {
    //         return Ok(());
    //     }
    //     index.list_chunks(
    //         &|_| true,  // no filter
    //         &|c| c.clone(),  // no map
    //         &|chunk| chunk.title.clone(),  // sort by title
    //     )?
    // } else {
    //     let query = uid_query(&index, &args, UidQueryConfig::new().file_or_chunk_only())?;
    //     let mut chunks = vec![];
    //     for uid in query.get_chunk_uids() {
    //         let chunk = index.get_chunk_by_uid(uid)?;
    //         chunks.push(chunk);
    //     }
    //     if chunks.is_empty() {
    //         for file_uid in query.get_file_uids() {
    //             let uids = index.get_chunks_of_file(file_uid)?;
    //             for uid in uids.iter() {
    //                 let chunk = index.get_chunk_by_uid(*uid)?;
    //                 chunks.push(chunk);
    //             }
    //         }
    //     }
    //     if chunks.is_empty() {
    //         return Err(anyhow::anyhow!(CliError::new_message(format!("There's no chunk/file that matches `{}`.", args.join(" ")))));
    //     }
    //     chunks.into_iter().map(|c| c.into()).collect()
    // };

    // if json_mode {
    //     if uid_only {
    //         println!("{}", serde_json::to_string_pretty(&chunks.iter().map(|c| c.title.clone()).collect::<Vec<_>>())?);
    //     } else {
    //         println!("{}", serde_json::to_string_pretty(&chunks)?);
    //     }
    // } else if uid_only {
    //     for chunk in chunks {
    //         println!("{}", chunk.title);
    //     }
    // }

    // Ok(())
}
