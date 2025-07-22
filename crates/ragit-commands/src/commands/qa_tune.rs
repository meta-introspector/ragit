use crate::prelude::*;

pub async fn qa_tune_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    println!("{}", get_doc_content("commands/qa_tune.txt"));
    Err(Error::CliError(CliError::new_message("qa_tune command is not implemented yet".to_string())))
}