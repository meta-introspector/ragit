use ragit_utils::prelude::*;
use ragit_api::prelude::*;
use ragit_types::prelude::*;
use ragit_index_io::index_struct::{Index};
use ragit_utils::project_root::find_root;
use ragit_utils::doc_utils::get_doc_content;
use ragit_types::ii_status::IIStatus;

pub fn ii_status_command_main(args: &[String]) -> Result<(), anyhow::Error> {
    let parsed_args = ArgParser::new().parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/ii-status.txt"));
        return Ok(());
    }

    let index = Index::load(find_root()?.into(), LoadMode::QuickCheck)?;

    let status = if index.ii_status.enabled {
        "complete" // Assuming enabled means complete for now
    } else {
        "not initialized" // Assuming not enabled means not initialized
    };
    println!("{status}");

    Ok(())
}
