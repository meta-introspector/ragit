use ragit_commands::{Error, pdl_command_main};

pub async fn pdl_command(args: &[String]) -> Result<(), Error> {
    pdl_command_main(args).await
}