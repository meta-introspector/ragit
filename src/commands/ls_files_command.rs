use ragit_commands::{Error, ls_files_command_main};

pub fn ls_files_command(args: &[String]) -> Result<(), Error> {
    ls_files_command_main(args).map_err(|e| e.into())
}
