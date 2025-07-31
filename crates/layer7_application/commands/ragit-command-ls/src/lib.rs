pub mod commands;

use anyhow::Error;
use ragit_cli::{ArgParser, ArgType, ArgCount};
use ragit_utils::prelude::*;
use ragit_utils::doc_utils::get_doc_content;

use crate::commands::ls_chunks::ls_chunks_command_main;
use crate::commands::ls_files::ls_files_command_main;
use crate::commands::ls_images::ls_images_command_main;
use crate::commands::ls_models::ls_models_command_main;
use crate::commands::ls_terms::ls_terms_command_main;

pub async fn ls_command_main(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .args(ArgType::Command, ArgCount::Leq(1))
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/ls.txt"));
        return Ok(());
    }

    let parsed_args_get_args = parsed_args.get_args();
    let command = parsed_args_get_args.get(0).map(|arg| arg.as_str());

    match command {
        Some("chunks") => ls_chunks_command_main(args).await?,
        Some("files") => ls_files_command_main(args).await?,
        Some("images") => ls_images_command_main(args).await?,
        Some("models") => ls_models_command_main(args).await?,
        Some("terms") => ls_terms_command_main(args).await?,
        _ => {
            println!("{}", get_doc_content("commands/ls.txt"));
        }
    }

    Ok(())
}