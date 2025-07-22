use ragit_commands::{Error, ls_terms_command_main};

pub fn ls_terms_command(args: &[String]) -> Result<(), Error> {
    ls_terms_command_main(args).map_err(|e| e.into())
}