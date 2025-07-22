use crate::prelude::*;

pub async fn version_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    println!("{}", get_doc_content("commands/version.txt"));
    Err(Error::CliError(CliError::new_message("version command is not implemented yet".to_string())))
}