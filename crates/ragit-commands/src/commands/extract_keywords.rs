use crate::prelude::*;

pub async fn extract_keywords_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    println!("{}", get_doc_content("commands/extract_keywords.txt"));
    Err(Error::CliError(CliError::new_message("extract_keywords command is not implemented yet".to_string())))
}