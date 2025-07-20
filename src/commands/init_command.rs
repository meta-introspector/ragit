use crate::{Error, Index};
use ragit_cli::ArgParser;

pub fn init_command(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new().parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", include_str!("../../docs/commands/init.txt"));
        return Ok(());
    }

    match Index::new(String::from(".")) {
        Ok(_) => {
            println!("initialized");
        }
        Err(Error::IndexAlreadyExists(_)) => {
            println!("There already is a knowledge-base here.");
        }
        Err(e) => {
            return Err(e);
        }
    }

    Ok(())
}
