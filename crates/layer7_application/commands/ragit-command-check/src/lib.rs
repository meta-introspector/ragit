use ragit_utils::prelude::*;
use ragit_utils::doc_utils::get_doc_content;
use ragit_utils::cli_types::CliError;
//use ragit_index_core::Index;

pub async fn check_command_main(_oargs: Vec<String>, _pre_args: ParsedArgs) -> Result<(), anyhow::Error> {
    println!("{}", get_doc_content("commands/check.txt"));
    Err(anyhow::anyhow!(CliError::new_message(
        "check command is not implemented yet".to_string(),
    )))
}
