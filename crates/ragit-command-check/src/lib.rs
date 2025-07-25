use ragit_utils::prelude::*;
use ragit_api::prelude::*;
use ragit_types::prelude::*;

pub async fn check_command_main(_oargs: Vec<String>, _pre_args: ParsedArgs) -> Result<(), Error> {
    println!("{}", get_doc_content("commands/check.txt"));
    Err(Error::CliError(CliError::new_message(
        "check command is not implemented yet".to_string(),
    )))
}
