use crate::prelude::*;

pub async fn status_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    println!("{}", get_doc_content("commands/status.txt"));
    Err(Error::CliError(CliError::new_message("status command is not implemented yet".to_string())))
}