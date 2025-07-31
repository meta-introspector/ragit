use ragit_utils::prelude::*;
use ragit_api::prelude::*;
use ragit_types::prelude::*;
// use ragit_index_io::load_index_from_path; // FIXME: load_index_from_path not found
// use ragit_index_core::{Index, LoadMode}; // FIXME: Index and LoadMode not found

pub async fn muse_summarize_command_main(args: &[String]) -> Result<(), anyhow::Error> {
    panic!("FIXME: muse_summarize_command_main not implemented");
/*
use ragit_utils::prelude::*;
use ragit_api::prelude::*;
use ragit_types::prelude::*;
use ragit_index_io::load_index_from_path;
use ragit_index_core::{Index, LoadMode};
use ragit_utils::project_root::find_root;
use ragit_types::summary::SummaryMode;

pub async fn muse_summarize_command_main(args: &[String]) -> Result<(), anyhow::Error> {
    let parsed_args = ArgParser::new()
        .optional_flag(&["--force", "--cached"])
        .parse(args, 2)?;

    if parsed_args.show_help() {
        // TODO: create a help file for muse-summarize
        println!("Usage: rag muse-summarize [--force | --cached]");
        return Ok(());
    }

    let root_dir = find_root()?;
    let mut index = load_index_from_path(&root_dir)?;
    index.api_config.enable_muse_mode = true;
    let summary_mode = if parsed_args.get_flag(0).is_some() {
        SummaryMode::Rerank
    } else {
        SummaryMode::Simple
    };

    let quiet = parsed_args.get_flag(1).is_some();
    let query = parsed_args.get_args();

    let summary = index.summary(Some(summary_mode)).await?;

    println!("{}", summary.unwrap_or_default());

    Ok(())
}
*/
    // let parsed_args = ArgParser::new()
    //     .optional_flag(&["--force", "--cached"])
    //     .parse(args, 2)?;

    // if parsed_args.show_help() {
    //     // TODO: create a help file for muse-summarize
    //     println!("Usage: rag muse-summarize [--force | --cached]");
    //     return Ok(());
    // }

    // let root_dir = find_root()?;
    // let mut index = load_index_from_path(&root_dir)?;
    // index.api_config.enable_muse_mode = true;
    // let summary_mode = if parsed_args.get_flag(0).is_some() {
    //     SummaryMode::Rerank
    // } else {
    //     SummaryMode::Simple
    // };

    // let quiet = parsed_args.get_flag(1).is_some();
    // let query = parsed_args.get_args();

    // let summary = index.summary(Some(summary_mode)).await?;

    // println!("{}", summary.unwrap_or_default());

    // Ok(())
}
