use ragit_utils::prelude::*;
use ragit_api::prelude::*;
use ragit_types::prelude::*;
//use ragit_index_io::load_index_from_path;
//use ragit_index_core::{Index, LoadMode};
use ragit_index_core::load_index_from_path;
use ragit_utils::project_root::find_root;
use ragit_utils::doc_utils::get_doc_content;

pub fn ii_build_command_main(args: &[String]) -> Result<(), anyhow::Error> {
    let parsed_args = ArgParser::new()
        .optional_flag(&["--quiet"])
        .short_flag(&["--quiet"])
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/ii-build.txt"));
        return Ok(());
    }

    let quiet = parsed_args.get_flag(0).is_some();
    let mut index = load_index_from_path(&find_root()?)?;
    index.build_ii(quiet)?;
    Ok(())
}
