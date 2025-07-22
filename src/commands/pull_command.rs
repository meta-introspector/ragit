use ragit_commands::{Error, pull_command_main};

pub async fn pull_command(args: &[String]) -> Result<(), Error> {
    pull_command_main(args).await
}