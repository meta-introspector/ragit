use ragit_commands::{Error, model_command_main};

pub async fn model_command(args: &[String]) -> Result<(), Error> {
    model_command_main(args).await
}