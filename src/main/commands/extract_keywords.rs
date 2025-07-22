use crate::prelude::*;

use crate::prelude::*;

use crate::commands::extract_keywords_command::extract_keywords_command;

pub async fn extract_keywords_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    extract_keywords_command(args, pre_args).await
}