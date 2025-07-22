use crate::prelude::*;

pub async fn summary_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    println!("{}", get_doc_content("commands/summary.txt"));
    Err(Error::CliError(CliError::new_message("summary command is not implemented yet".to_string())))
}