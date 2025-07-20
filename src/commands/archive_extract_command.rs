use ragit::{Error, Index, Path};
use ragit_cli::{ArgCount, ArgParser, ArgType};

pub async fn archive_extract_command(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .arg_flag_with_default("--jobs", "4", ArgType::IntegerBetween { min: Some(0), max: None })
        .arg_flag("--output", ArgType::Path)
        .optional_flag(&["--force"])
        .optional_flag(&["--quiet"])
        .short_flag(&["--force", "--output", "--quiet"])
        .args(ArgType::Path, ArgCount::Geq(1))
        .parse(&args, 2)?;

    if parsed_args.show_help() {
        println!("{}", include_str!("../../docs/commands/archive-extract.txt"));
        return Ok(());
    }

    let jobs = parsed_args.arg_flags.get("--jobs").as_ref().unwrap().parse::<usize>().unwrap();
    let output = parsed_args.arg_flags.get("--output").as_ref().unwrap().to_string();
    let archives = parsed_args.get_args();
    let force = parsed_args.get_flag(0).is_some();
    let quiet = parsed_args.get_flag(1).is_some();
    Index::extract_archive(
        &output,
        archives,
        jobs,
        force,
        quiet,
    )?;
    Ok(())
}
