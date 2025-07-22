use crate::prelude::*;

pub async fn init_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    println!("{}", get_doc_content("commands/init.txt"));
    Err(Error::CliError(CliError::new_message("init command is not implemented yet".to_string())))
}