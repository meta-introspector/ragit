use crate::prelude::*;

pub async fn push_command_main(args: Vec<String>, _pre_args: ParsedArgs) -> Result<(), Error> {
    println!("{}", get_doc_content("commands/push.txt"));
    Err(Error::CliError(CliError::new_message("push command is not implemented yet".to_string())))
}