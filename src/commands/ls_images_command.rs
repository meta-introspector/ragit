use ragit_commands::{Error, ls_images_command_main};

pub fn ls_images_command(args: &[String]) -> Result<(), Error> {
    ls_images_command_main(args).map_err(|e| e.into())
}
