use ragit::{Error, Index, Path};
use ragit_cli::{ArgCount, ArgParser, ArgType};

pub async fn clone_command(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .optional_arg_flag("--depth", ArgType::IntegerBetween { min: Some(0), max: None })
        .args(ArgType::Path, ArgCount::Fixed(1))
        .parse(args, 1)?;

    if parsed_args.show_help() {
        println!("{}", include_str!("../../docs/commands/clone.txt"));
        return Ok(());
    }

    let url = parsed_args.get_arg(0).unwrap();
    let depth = parsed_args.arg_flags.get("--depth").map(|s| s.parse::<usize>().unwrap());

    let mut index = Index::new();
    index.clone(url, depth).await?;

    Ok(())
}
