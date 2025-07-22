use crate::prelude::*;
use ragit_utils::index::commands::qa_test_command;

pub async fn qa_test_command_main(args: Vec<String>, pre_args: ragit_cli::ParsedArgs) -> Result<(), Error> {
    qa_test_command(args, pre_args).await
}