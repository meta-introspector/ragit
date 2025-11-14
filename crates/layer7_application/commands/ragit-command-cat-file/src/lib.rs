use ragit_utils::prelude::*;
use ragit_utils::doc_utils::get_doc_content;
use ragit_utils::cli_types::CliError;

pub async fn cat_file_command_main(_args: Vec<String>, _pre_args: ParsedArgs) -> Result<(), anyhow::Error> {
    println!("{}", get_doc_content("commands/cat_file.txt"));
    Err(anyhow::anyhow!(CliError::new_message(
        "cat_file command is not implemented yet".to_string(),
    )))
}