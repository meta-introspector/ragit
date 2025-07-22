use ragit::{Error, Index, Path};
use ragit_cli::{ArgCount, ArgParser, ArgType};

pub async fn clone_command(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .optional_flag(&["--quiet"])
        .short_flag(&["--quiet"])
        .args(ArgType::Url, ArgCount::Geq(1))
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", include_str!("../../docs/commands/clone.txt"));
        return Ok(());
    }

    if crate::find_root().is_ok() {
        return Err(Error::CannotClone(String::from("You're already inside a knowledge-base. You cannot clone another knowledge-base here.")));
    }

    let args = parsed_args.get_args();
    let quiet = parsed_args.get_flag(0).is_some();
    Index::clone(
        args[0].clone(),
        args.get(1).map(|s| s.to_string()),
        quiet,
        None,
    ).await?;
    let mut index = Index::load(crate::find_root()?.to_string_lossy().into_owned(), LoadMode::QuickCheck)?;
    index.save_to_file(index.root_dir.join(INDEX_FILE_NAME.to_string()))?;
    Ok(())
}