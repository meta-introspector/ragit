use crate::prelude::*;

pub fn version_command_main(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new().parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/version.txt"));
        return Ok(());
    }

    println!("ragit version {}", env!("CARGO_PKG_VERSION"));
    println!("build options: {:?}", get_build_options());

    Ok(())
}