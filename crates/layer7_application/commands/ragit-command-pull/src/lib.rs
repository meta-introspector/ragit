use ragit_utils::prelude::*;
use ragit_api::prelude::*;
use ragit_types::prelude::*;
// use ragit_index_core::load_index_from_path; // FIXME: load_index_from_path not found
//use ragit_index_core::Index;
use ragit_utils::project_root::find_root;
use ragit_utils::doc_utils::get_doc_content;

pub async fn pull_command_main(args: &[String]) -> Result<(), anyhow::Error> {
    panic!("FIXME: pull_command_main not implemented");
/*
use ragit_utils::prelude::*;
use ragit_api::prelude::*;
use ragit_types::prelude::*;
use ragit_index_core::load_index_from_path;
//use ragit_index_core::Index;
use ragit_utils::project_root::find_root;
use ragit_utils::doc_utils::get_doc_content;

pub async fn pull_command_main(args: &[String]) -> Result<(), anyhow::Error> {
    let parsed_args = ArgParser::new()
        .flag_with_default(&["--no-configs", "--configs"])
        .flag_with_default(&["--no-prompts", "--prompts"])
        .optional_flag(&["--quiet"])
        .short_flag(&["--quiet"])
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/pull.txt"));
        return Ok(());
    }

    let mut index = load_index_from_path(&find_root()?)?;
    let include_configs = parsed_args.get_flag(0).unwrap() == "--configs";
    let include_prompts = parsed_args.get_flag(1).unwrap() == "--prompts";
    let quiet = parsed_args.get_flag(2).is_some();
    index.pull(include_configs, include_prompts, quiet).await?;

    println!("Already up to date.");

    Ok(())
}
*/
    // let parsed_args = ArgParser::new()
    //     .flag_with_default(&["--no-configs", "--configs"])
    //     .flag_with_default(&["--no-prompts", "--prompts"])
    //     .optional_flag(&["--quiet"])
    //     .short_flag(&["--quiet"])
    //     .parse(args, 2)?;

    // if parsed_args.show_help() {
    //     println!("{}", get_doc_content("commands/pull.txt"));
    //     return Ok(());
    // }

    // let mut index = load_index_from_path(&find_root()?)?;
    // let include_configs = parsed_args.get_flag(0).unwrap() == "--configs";
    // let include_prompts = parsed_args.get_flag(1).unwrap() == "--prompts";
    // let quiet = parsed_args.get_flag(2).is_some();
    // index.pull(include_configs, include_prompts, quiet).await?;

    // println!("Already up to date.");

    // Ok(())
}
