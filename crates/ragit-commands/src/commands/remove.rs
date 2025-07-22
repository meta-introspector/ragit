use crate::prelude::*;

pub async fn remove_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    println!("{}", get_doc_content("commands/remove.txt"));
    Err(Error::CliError(CliError::new_message("remove command is not implemented yet".to_string())))
}