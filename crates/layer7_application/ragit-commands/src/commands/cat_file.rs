use crate::prelude::*;

pub async fn cat_file_command_main(_args: Vec<String>, _pre_args: ParsedArgs) -> Result<(), Error> {
    println!("{}", get_doc_content("commands/cat_file.txt"));
    Err(Error::CliError(CliError::new_message(
        "cat_file command is not implemented yet".to_string(),
    )))
}
