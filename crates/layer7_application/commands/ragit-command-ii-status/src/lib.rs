use ragit_utils::prelude::*;
use ragit_index_types::index_struct::Index;
// use ragit_index_io::load_index_from_path; // FIXME: load_index_from_path not found
use ragit_index_types::load_mode::LoadMode;

use ragit_utils::project_root::find_root;
use ragit_utils::doc_utils::get_doc_content;
use ragit_types::ii_status::IIStatus;

pub fn ii_status_command_main(args: &[String]) -> Result<(), anyhow::Error> {
    let parsed_args = ArgParser::new().parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/ii-status.txt"));
        return Ok(());
    }

    use ragit_index_types::load_mode::LoadMode;
    let index = Index::load(find_root()?.into(), LoadMode::QuickCheck)?;

    panic!("FIXME: IIStatus enabled field removed");

    Ok(())
}

/*
    let status = if index.ii_status.enabled {
        "complete" // Assuming enabled means complete for now
    } else {
        "not initialized" // Assuming not enabled means not initialized
    };
    println!("{status}");
*/
