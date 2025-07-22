use crate::prelude::*;

pub async fn query_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    println!("{}", get_doc_content("commands/query.txt"));
    Err(Error::CliError(CliError::new_message("query command is not implemented yet".to_string())))
}