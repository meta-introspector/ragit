use crate::prelude::*;

pub async fn ls_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    println!("{}", get_doc_content("commands/ls.txt"));
    Err(Error::CliError(CliError::new_message("ls command is not implemented yet".to_string())))
}