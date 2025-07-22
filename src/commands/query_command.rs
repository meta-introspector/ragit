use ragit_commands::{Error, query_command_main};

pub async fn query_command(args: &[String]) -> Result<(), Error> {
    query_command_main(args).await
}