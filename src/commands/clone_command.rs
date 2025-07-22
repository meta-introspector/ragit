use ragit_utils::error::Error;
use ragit_utils::index::index_struct::Index;
use std::path::PathBuf;
use ragit_utils::cli_types::{ArgCount, ArgParser, ArgType};

pub async fn clone_command(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .optional_arg_flag("--depth", ArgType::IntegerBetween { min: Some(0), max: None })
        .args(ArgType::Path, ArgCount::Exact(1))
        .parse(args, 1)?;

    if parsed_args.show_help() {
        println!("{}", include_str!("../../docs/commands/clone.txt"));
        return Ok(());
    }

    let url = parsed_args.get_args_exact(1)?.get(0).unwrap();
    let depth = parsed_args.arg_flags.get("--depth").map(|s| s.parse::<usize>().unwrap());

    let mut index = Index::new(PathBuf::from("."))?;
    index.clone(url, depth).await?;

    Ok(())
}
