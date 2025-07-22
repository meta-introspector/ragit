use ragit_utils::error::Error;
use ragit_utils::index::index_struct::Index;
use ragit_utils::index::load_mode::LoadMode;
use std::path::PathBuf;
use ragit_utils::cli_types::{ArgCount, ArgParser, ArgType};
use ragit_utils::project_root::find_root;

pub async fn build_command(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .arg_flag_with_default("--jobs", "8", ArgType::IntegerBetween { min: Some(0), max: None })
        .optional_flag(&["--quiet"])
        .short_flag(&["--quiet"])
        .parse(&args, 2)?;

    if parsed_args.show_help() {
        println!("{}", include_str!("../../docs/commands/build.txt"));
        return Ok(());
    }

    let jobs = parsed_args.arg_flags.get("--jobs").as_ref().unwrap().parse::<usize>().unwrap();
    let quiet = parsed_args.get_flag(0).is_some();
    let mut index = Index::load(find_root()?.to_string_lossy().into_owned(), LoadMode::QuickCheck)?;
    index.build(jobs, quiet).await?;
    Ok(())
}
