use crate::prelude::*;

pub async fn model_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    println!("{}", get_doc_content("commands/model.txt"));
    Err(Error::CliError(CliError::new_message("model command is not implemented yet".to_string())))
}