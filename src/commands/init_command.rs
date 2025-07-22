use ragit_commands::init_command_main;
use ragit_utils::error::Error;

pub fn init_command(args: &[String]) -> Result<(), Error> {
    init_command_main(args).map_err(|e| e.into())
}