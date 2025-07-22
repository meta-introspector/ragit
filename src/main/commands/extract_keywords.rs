use crate::prelude::*;

use ragit_utils::index::commands::extract_keywords::extract_keywords_command;

pub async fn extract_keywords_command_main(args: Vec<String>, pre_args: ragit_args::ParsedArgs) -> Result<(), Error> {
    extract_keywords_command(args, pre_args).await
}