use ragit_utils::prelude::*;
use ragit_api::prelude::*;
use ragit_types::prelude::*;
use ragit_index::Index;
use ragit_index_core::LoadMode;
use ragit_utils::project_root::find_root;
use ragit_utils::doc_utils::get_doc_content;

pub fn ii_reset_command_main(args: &[String]) -> Result<(), anyhow::Error> {
    let parsed_args = ArgParser::new().parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/ii-reset.txt"));
        return Ok(());
    }

    let mut index = Index::load(find_root()?.into(), LoadMode::QuickCheck)?;
    index.reset_ii()?;
    Ok(())
}
