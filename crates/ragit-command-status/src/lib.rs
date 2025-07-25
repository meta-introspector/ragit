use ragit_utils::prelude::*;
//use ragit_cli::prelude::*;
//use ragit_index_io::load_index_from_path;
//use ragit_index_core::Index;
use ragit_index_io::index_struct::Index;
use ragit_utils::project_root::find_root;
use ragit_utils::doc_utils::get_doc_content;
use ragit_index_io::index_struct::load_index_from_path;
pub fn status_command_main(args: &[String]) -> Result<(), anyhow::Error> {
    let parsed_args = ArgParser::new().parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/status.txt"));
        return Ok(());
    }

    let index = load_index_from_path(&find_root()?)?;

    let status = index.status()?;

    println!("{status}");

    Ok(())
}
