use ragit_commands::{Error, qa_test_command_main};

pub async fn qa_test_command(args: &[String]) -> Result<(), Error> {
    qa_test_command_main(args).await
}