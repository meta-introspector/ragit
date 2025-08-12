pub async fn push_command_main(args: &[String]) -> Result<(), anyhow::Error> {
    panic!("FIX ME LATER: ragit-command-push is commented out.");
/*
use ragit_utils::prelude::*;
use ragit_api::prelude::*;
use ragit_types::prelude::*;
use ragit_index_io::index_struct::Index;
use ragit_utils::project_root::find_root;
use ragit_utils::doc_utils::get_doc_content;
use ragit_index_core::PushResult;
use ragit_index_core::load_index_from_path;

pub async fn push_command_main(args: &[String]) -> Result<(), anyhow::Error> {
    let parsed_args = ArgParser::new()
        .optional_arg_flag("--remote", ArgType::Path)
        .flag_with_default(&["--no-configs", "--configs"])
        .flag_with_default(&["--no-prompts", "--prompts"])
        .optional_flag(&["--quiet"])
        .short_flag(&["--quiet"])
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/push.txt"));
        return Ok(());
    }

    let mut index = load_index_from_path(&find_root()?)?;
    let remote = parsed_args.arg_flags.get("--remote").map(|s| s.to_string());
    let include_configs = parsed_args.get_flag(0).unwrap() == "--configs";
    let include_prompts = parsed_args.get_flag(1).unwrap() == "--prompts";
    let quiet = parsed_args.get_flag(2).is_some();
    let result = index.push(include_configs, include_prompts, quiet).await?;

    match result {
        PushResult::AlreadyUpToDate => {
            println!("Everything up-to-date");
        }
        _ => {}
    }

    Ok(())
}
*/
}