use ragit_commands::{Error, push_command_main};

pub async fn push_command(args: &[String]) -> Result<(), Error> {
    push_command_main(args).await
}