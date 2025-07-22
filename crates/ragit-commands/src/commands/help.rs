use crate::prelude::*;

pub async fn help_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    println!("{}", get_doc_content("commands/help.txt"));
    Err(Error::CliError(CliError::new_message("help command is not implemented yet".to_string())))
}