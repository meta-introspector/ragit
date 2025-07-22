use crate::prelude::*;

pub async fn extract_keywords_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    crate::index::commands::extract_keywords::extract_keywords_command(args, pre_args).await
}