use ragit::{Error, Index, LoadMode, Path};
use ragit_cli::{ArgCount, ArgParser, ArgType};

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
    let mut index = Index::load(crate::find_root()?.to_string_lossy().into_owned(), LoadMode::QuickCheck)?;
    index.build(jobs, quiet).await?;
    Ok(())
}
