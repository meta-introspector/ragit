use crate::prelude::*;

pub async fn push_command_main(_args: &[String]) -> Result<(), Error> {
    // let parsed_args = ArgParser::new()
    //     .optional_arg_flag("--remote", ArgType::Path)
    //     .flag_with_default(&["--no-configs", "--configs"])
    //     .flag_with_default(&["--no-prompts", "--prompts"])
    //     .optional_flag(&["--quiet"])
    //     .short_flag(&["--quiet"])
    //     .parse(args, 2)?;

    // if parsed_args.show_help() {
    //     println!("{}", get_doc_content("commands/push.txt"));
    //     return Ok(());
    // }

    // let index = Index::load(find_root()?.into(), LoadMode::QuickCheck)?;
    // let remote = parsed_args.arg_flags.get("--remote").map(|s| s.to_string());
    // let include_configs = parsed_args.get_flag(0).unwrap() == "--configs";
    // let include_prompts = parsed_args.get_flag(1).unwrap() == "--prompts";
    // let quiet = parsed_args.get_flag(2).is_some();
    // let result = index.push(include_configs, include_prompts, quiet).await?;

    // match result {
    //     PushResult::AlreadyUpToDate => {
    //         println!("Everything up-to-date");
    //     }
    //     _ => {}
    // }

    Ok(())
}
