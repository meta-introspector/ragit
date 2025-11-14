use ragit_commands::{Error, ls_models_command_main};

pub fn ls_models_command(args: &[String]) -> Result<(), Error> {
    ls_models_command_main(args).map_err(|e| e.into())
}