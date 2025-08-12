use ragit_commands::{Error, remove_command_main};

pub fn remove_command(args: &[String]) -> Result<(), Error> {
    remove_command_main(args).map_err(|e| e.into())
}