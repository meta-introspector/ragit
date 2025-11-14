use ragit_commands::{Error, version_command_main};

pub fn version_command(args: &[String]) -> Result<(), Error> {
    version_command_main(args).map_err(|e| e.into())
}