use ragit_utils::index::load_mode::LoadMode;
use crate::prelude::*;
use crate::main::find_root;
use crate::commands::push_command::push_command;

pub async fn push_command_main(args: Vec<String>, _pre_args: ParsedArgs) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .optional_arg_flag("--remote", ArgType::String)
        .optional_flag(&["--quiet"])
        .parse(&args, 2)?;

    if parsed_args.show_help() {
        println!("{}", include_str!("../../../docs/commands/push.txt"));
        return Ok(());
    }

    let root_dir = find_root()?;
    let index = Index::load(root_dir, LoadMode::QuickCheck)?;
    let remote = parsed_args.arg_flags.get("--remote").map(|s| s.to_string());
    let quiet = parsed_args.get_flag(0).is_some();

    let remote_url = match remote {
        Some(url) => url,
        None => match index.repo_url {
            Some(url) => url,
            None => return Err(Error::CannotPush(String::from("Please specify where to push."))),
        },
    };

    push_command(&index, remote_url, quiet).await
}