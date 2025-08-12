use ragit_commands::config_command_main;
use ragit_utils::error::Error;

pub fn config_command(args: &[String]) -> Result<(), Error> {
    config_command_main(args).map_err(|e| e.into())
}