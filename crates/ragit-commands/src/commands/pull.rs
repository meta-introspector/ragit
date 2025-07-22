use crate::prelude::*;

pub async fn pull_command_main(args: Vec<String>, _pre_args: ParsedArgs) -> Result<(), Error> {
    println!("{}", get_doc_content("commands/pull.txt"));
    Err(Error::CliError(CliError::new_message("pull command is not implemented yet".to_string())))
}