use crate::prelude::*;

pub fn status_command_main(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new().parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/status.txt"));
        return Ok(());
    }

    let index = Index::load(find_root()?.into(), LoadMode::OnlyJson)?;
    let status = index.status()?;

    println!("{status}");

    Ok(())
}