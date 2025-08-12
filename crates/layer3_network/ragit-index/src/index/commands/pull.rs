use crate::prelude::*;
use ragit_index_types::Index;

pub async fn pull_command(root_dir: PathBuf, args: &[String]) -> Result<(), ApiError> {
    let parsed_args = ArgParser::new()
        .optional_arg_flag("--include-configs", ArgType::String)
        .optional_arg_flag("--include-prompts", ArgType::String)
        .optional_arg_flag("--quiet", ArgType::String)
        .parse(args, 2)?;

    let include_configs = parsed_args.arg_flags.get("--include-configs").is_some();
    let include_prompts = parsed_args.arg_flags.get("--include-prompts").is_some();
    let quiet = parsed_args.arg_flags.get("--quiet").is_some();

    let mut index = Index::load(root_dir, LoadMode::QuickCheck)?;
    // TODO: Implement pull functionality
    let result = FetchResult { fetched: 0, updated: 0 };

    if !quiet {
        println!("fetched: {}, updated: {}", result.fetched, result.updated);
    }

    Ok(())
}