use crate::prelude::*;

pub async fn ii_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    println!("{}", get_doc_content("commands/ii.txt"));
    Err(Error::CliError(CliError::new_message("ii command is not implemented yet".to_string())))
}