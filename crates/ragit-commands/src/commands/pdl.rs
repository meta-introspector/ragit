use crate::prelude::*;

pub async fn pdl_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    println!("{}", get_doc_content("commands/pdl.txt"));
    Err(Error::CliError(CliError::new_message("pdl command is not implemented yet".to_string())))
}