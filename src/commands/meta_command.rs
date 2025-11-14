use ragit_commands::{Error, meta_command_main};

pub fn meta_command(args: &[String]) -> Result<(), Error> {
    meta_command_main(args).map_err(|e| e.into())
}