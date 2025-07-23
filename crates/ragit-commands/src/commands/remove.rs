use crate::prelude::*;

pub fn remove_command_main(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .optional_flag(&["--dry-run"])
        .optional_flag(&["--recursive"])
        .optional_flag(&["--auto"])
        .optional_flag(&["--staged"])
        .optional_flag(&["--processed"])
        .short_flag(&["--recursive"])
        .args(ArgType::UidOrPath, ArgCount::Geq(1))
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/remove.txt"));
        return Ok(());
    }

    let mut index = Index::load(find_root()?.into(), LoadMode::OnlyJson)?;
    let dry_run = parsed_args.get_flag(0).is_some();
    let recursive = parsed_args.get_flag(1).is_some();
    let auto = parsed_args.get_flag(2).is_some();
    let staged = parsed_args.get_flag(3).is_some();
    let processed = parsed_args.get_flag(4).is_some();
    let query = parsed_args.get_args();

    let result = index.remove_files(&query, dry_run, recursive, auto, staged, processed)?;

    println!("{result}");

    Ok(())
}
