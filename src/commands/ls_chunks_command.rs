use ragit_commands::{Error, ls_chunks_command_main};

pub fn ls_chunks_command(args: &[String]) -> Result<(), Error> {
    ls_chunks_command_main(args).map_err(|e| e.into())
}
