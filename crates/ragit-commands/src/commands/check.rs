use crate::prelude::*;

pub async fn check_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    println!("{}", get_doc_content("commands/check.txt"));
    Err(Error::CliError(CliError::new_message("check command is not implemented yet".to_string())))
}