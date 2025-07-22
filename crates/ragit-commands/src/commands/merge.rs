use crate::prelude::*;

pub async fn merge_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    println!("{}", get_doc_content("commands/merge.txt"));
    Err(Error::CliError(CliError::new_message("merge command is not implemented yet".to_string())))
}