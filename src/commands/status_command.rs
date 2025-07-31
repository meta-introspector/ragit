use ragit_commands::{Error, status_command_main};

pub fn status_command(args: &[String]) -> Result<(), Error> {
    status_command_main(args).map_err(|e| e.into())
}