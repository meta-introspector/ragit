use ragit::{Error, Index, LoadMode, Path};
use ragit_cli::{ArgCount, ArgParser, ArgType};

pub async fn archive_create_command(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .arg_flag_with_default("--jobs", "4", ArgType::IntegerBetween { min: Some(0), max: None })
        .optional_arg_flag("--size-limit", ArgType::IntegerBetween { min: Some(0), max: None })
        .arg_flag("--output", ArgType::Path)
        .flag_with_default(&["--no-configs", "--configs"])
        .flag_with_default(&["--no-prompts", "--prompts"])
        .optional_flag(&["--force"])
        .optional_flag(&["--quiet"])
        .short_flag(&["--force", "--output", "--quiet"])
        .parse(&args, 2)?;

    if parsed_args.show_help() {
        println!("{}", include_str!("../../docs/commands/archive-create.txt"));
        return Ok(());
    }

    let index = Index::load(crate::find_root()?.to_string_lossy().into_owned(), LoadMode::QuickCheck)?;
    let jobs = parsed_args.arg_flags.get("--jobs").as_ref().unwrap().parse::<usize>().unwrap();
    let size_limit = parsed_args.arg_flags.get("--size-limit").as_ref().map(|n| n.parse::<u64>().unwrap());
    let output = parsed_args.arg_flags.get("--output").as_ref().unwrap().to_string();
    let include_configs = parsed_args.get_flag(0).unwrap() == "--configs";
    let include_prompts = parsed_args.get_flag(1).unwrap() == "--prompts";
    let force = parsed_args.get_flag(2).is_some();
    let quiet = parsed_args.get_flag(3).is_some();
    index.create_archive(
        jobs,
        size_limit,
        output,
        include_configs,
        include_prompts,
        force,
        quiet,
    )?;
    Ok(())
}
