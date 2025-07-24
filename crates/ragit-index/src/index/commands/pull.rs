use crate::prelude::*;

pub async fn pull_command(root_dir: PathBuf, args: &[String]) -> Result<(), ApiError> {
    let parsed_args = ArgParser::new()
        .optional_arg_flag("--include-configs", ArgType::Bool)
        .optional_arg_flag("--include-prompts", ArgType::Bool)
        .optional_arg_flag("--quiet", ArgType::Bool)
        .parse(args, 2)?;

    let include_configs = parsed_args.get_optional_arg_flag("--include-configs").unwrap_or(false);
    let include_prompts = parsed_args.get_optional_arg_flag("--include-prompts").unwrap_or(false);
    let quiet = parsed_args.get_optional_arg_flag("--quiet").unwrap_or(false);

    let mut index = Index::load(root_dir, LoadMode::QuickCheck)?;
    // TODO: Implement pull functionality
    let result = FetchResult { fetched: 0, updated: 0 };

    if !quiet {
        println!("fetched: {}, updated: {}", result.fetched, result.updated);
    }

    Ok(())
}