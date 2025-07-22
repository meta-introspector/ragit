use crate::{Error, Index, LoadMode};
use ragit_cli::ArgParser;

pub fn status_command(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new().parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", include_str!("../../docs/commands/status.txt"));
        return Ok(());
    }

    let index = Index::load(crate::find_root()?.to_string_lossy().into_owned(), LoadMode::OnlyJson)?;
    let status = index.status()?;

    println!("{status}");

    Ok(())
}
