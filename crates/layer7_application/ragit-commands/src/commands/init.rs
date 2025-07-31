use crate::prelude::*;

pub fn init_command_main(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new().parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/init.txt"));
        return Ok(());
    }

    match Index::new(".".into()) {
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
