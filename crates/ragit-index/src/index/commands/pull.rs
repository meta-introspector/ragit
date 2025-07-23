// use crate::cli_types::ArgParser;
// use crate::error::Error;
// use crate::index::index_struct::Index;
// use crate::index::load_mode::LoadMode;
// use std::path::PathBuf;

pub async fn pull_command(root_dir: PathBuf, args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .flag_with_default(&["--no-configs", "--configs"])
        .flag_with_default(&["--no-prompts", "--prompts"])
        .optional_flag(&["--quiet"])
        .short_flag(&["--quiet"])
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", include_str!("../../../../../docs/commands/pull.txt"));
        return Ok(());
    }

    let mut index = Index::load(root_dir, LoadMode::QuickCheck)?;
    let include_configs = parsed_args.get_flag(0).unwrap() == "--configs";
    let include_prompts = parsed_args.get_flag(1).unwrap() == "--prompts";
    let quiet = parsed_args.get_flag(2).is_some();
    let result = index.pull(include_configs, include_prompts, quiet).await?;

    match result {
        PullResult::AlreadyUpToDate => {
            println!("Already up to date.");
        }
        _ => {}
    }

    Ok(())
}

pub enum PullResult {
    AlreadyUpToDate,
    Pulled,
}
