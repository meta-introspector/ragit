use ragit_utils::prelude::*;
// use ragit_index_core::load_index_from_path;
// use ragit_index_core::index_struct::Index;
// use ragit_query::query_helpers::{uid_query, UidQueryConfig};
// use ragit_index_io::get_image_schema;

pub async fn ls_images_command_main(_args: &[String]) -> Result<(), anyhow::Error> {
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
use ragit_types::image::ImageSchema;
use ragit_index_io::get_image_schema;

pub async fn ls_images_command_main(args: &[String]) -> Result<(), anyhow::Error> {
    let parsed_args = ArgParser::new()
        .optional_flag(&["--uid-only", "--stat-only"])
        .optional_flag(&["--json"])
        .short_flag(&["--json"])
        .args(ArgType::UidOrPath, ArgCount::Any)
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/ls-images.txt"));
        return Ok(());
    }

    let uid_only = parsed_args.get_flag(0).unwrap_or_default() == "--uid-only";
    let stat_only = parsed_args.get_flag(0).unwrap_or_default() == "--stat-only";
    let json_mode = parsed_args.get_flag(1).is_some();
    let index = load_index_from_path(&find_root()?)?;
    let args = parsed_args.get_args();
    let images = if args.is_empty() {
        index.list_images(
            &|_| true,  // no filter
            &|image| image.clone(),  // no map
            &|_| 0,  // no sort
        )?
    } else {
        let query = uid_query(&index, &args, UidQueryConfig::new())?;
        let mut image_uids = vec![];
        for (_, uid) in query.get_processed_files() {
            for image_uid in index.get_images_of_file(uid)? {
                image_uids.push(image_uid);
            }
        }
        for uid in query.get_chunk_uids() {
            let chunk = index.get_chunk_by_uid(uid)?;
            // chunk.images is not available in ChunkSchema
        }
        for image_uid in query.get_image_uids() {
            image_uids.push(image_uid);
        }
        if image_uids.is_empty() {
            return Err(anyhow::anyhow!(CliError::new_message(format!("There's no chunk/file/image that matches `{}`.", args.join(" ")))));
        }
        let mut result = Vec::with_capacity(image_uids.len());
        for image_uid in image_uids.iter() {
            result.push(ragit_schema::get_image_schema(&index, *image_uid, false)?);
        }
        result
    };

    if uid_only {
        if json_mode {
            println!("{}", serde_json::to_string_pretty(&images.iter().map(|image| image.uid).collect::<Vec<_>>())?);
        } else {
            println!("{}", serde_json::to_string_pretty(&images)?);
        }
    } else {
        for image in images.iter() {
            println!("----------");
            println!("uid: {}", image.uid);
            println!("description: {}", image.description);
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
    //     println!("{}", get_doc_content("commands/ls-images.txt"));
    //     return Ok(());
    // }

    // let uid_only = parsed_args.get_flag(0).unwrap_or_default() == "--uid-only";
    // let stat_only = parsed_args.get_flag(0).unwrap_or_default() == "--stat-only";
    // let json_mode = parsed_args.get_flag(1).is_some();
    // let index = load_index_from_path(&find_root()?)?;
    // let args = parsed_args.get_args();
    // let images = if args.is_empty() {
    //     index.list_images(
    //         &|_| true,  // no filter
    //         &|image| image.clone(),  // no map
    //         &|_| 0,  // no sort
    //     )?
    // } else {
    //     let query = uid_query(&index, &args, UidQueryConfig::new())?;
    //     let mut image_uids = vec![];
    //     for (_, uid) in query.get_processed_files() {
    //         for image_uid in index.get_images_of_file(uid)? {
    //             image_uids.push(image_uid);
    //         }
    //     }
    //     for uid in query.get_chunk_uids() {
    //         let chunk = index.get_chunk_by_uid(uid)?;
    //         // chunk.images is not available in ChunkSchema
    //     }
    //     for image_uid in query.get_image_uids() {
    //         image_uids.push(image_uid);
    //     }
    //     if image_uids.is_empty() {
    //         return Err(anyhow::anyhow!(CliError::new_message(format!("There's no chunk/file/image that matches `{}`.", args.join(" ")))));
    //     }
    //     let mut result = Vec::with_capacity(image_uids.len());
    //     for image_uid in image_uids.iter() {
    //         result.push(ragit_schema::get_image_schema(&index, *image_uid, false)?);
    //     }
    //     result
    // };

    // if uid_only {
    //     if json_mode {
    //         println!("{}", serde_json::to_string_pretty(&images.iter().map(|image| image.uid).collect::<Vec<_>>())?);
    //     } else {
    //         println!("{}", serde_json::to_string_pretty(&images)?);
    //     }
    // } else {
    //     for image in images.iter() {
    //         println!("----------");
    //         println!("uid: {}", image.uid);
    //         println!("description: {}", image.description);
    //     }
    // }

    // Ok(())
}