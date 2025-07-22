use crate::prelude::*;

pub async fn cat_file_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    println!("{}", get_doc_content("commands/cat_file.txt"));
    Err(Error::CliError(CliError::new_message("cat_file command is not implemented yet".to_string())))
}