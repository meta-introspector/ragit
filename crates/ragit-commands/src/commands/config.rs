use crate::prelude::*;

pub async fn config_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    println!("{}", get_doc_content("commands/config.txt"));
    Err(Error::CliError(CliError::new_message("config command is not implemented yet".to_string())))
}