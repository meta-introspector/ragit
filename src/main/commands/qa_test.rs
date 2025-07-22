use crate::prelude::*;
use crate::commands::qa_test_command::qa_test_command;

pub async fn qa_test_command_main(args: Vec<String>, pre_args: ParsedArgs) -> Result<(), Error> {
    qa_test_command(args, pre_args).await
}