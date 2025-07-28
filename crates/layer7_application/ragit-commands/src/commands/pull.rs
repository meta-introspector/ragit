use crate::prelude::*;

pub async fn pull_command_main(args: &[String]) -> Result<(), Error> {
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

    let index = Index::load(find_root()?.into(), LoadMode::QuickCheck)?;
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
