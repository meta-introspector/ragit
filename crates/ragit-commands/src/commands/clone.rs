use crate::prelude::*;

pub async fn clone_command_main(args: Vec<String>, _pre_args: ParsedArgs) -> Result<(), Error> {
    println!("{}", get_doc_content("commands/clone.txt"));
    Err(Error::CliError(CliError::new_message("clone command is not implemented yet".to_string())))
}