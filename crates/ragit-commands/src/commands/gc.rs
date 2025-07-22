use crate::prelude::*;

pub async fn gc_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    println!("{}", get_doc_content("commands/gc.txt"));
    Err(Error::CliError(CliError::new_message("gc command is not implemented yet".to_string())))
}