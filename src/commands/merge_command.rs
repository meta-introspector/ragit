use ragit_commands::{Error, merge_command_main};

pub async fn merge_command(args: &[String]) -> Result<(), Error> {
    merge_command_main(args).await
}