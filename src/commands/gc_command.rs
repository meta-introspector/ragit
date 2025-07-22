use ragit_commands::gc_command_main;
use ragit_utils::error::Error;

pub fn gc_command(args: &[String]) -> Result<(), Error> {
    gc_command_main(args).map_err(|e| e.into())
}