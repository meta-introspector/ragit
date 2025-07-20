use crate::{get_build_options, Error};
use ragit_cli::ArgParser;

pub fn version_command(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new().parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", include_str!("../../docs/commands/version.txt"));
        return Ok(());
    }

    println!("ragit version {}", ragit::VERSION);
    println!("build options: {:?}", get_build_options());

    Ok(())
}
