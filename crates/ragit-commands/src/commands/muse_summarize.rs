use crate::prelude::*;

pub async fn muse_summarize_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    println!("{}", get_doc_content("commands/muse_summarize.txt"));
    Err(Error::CliError(CliError::new_message("muse_summarize command is not implemented yet".to_string())))
}