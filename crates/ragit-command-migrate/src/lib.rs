use ragit_utils::prelude::*;
//use ragit_cli::prelude::*;
//use ragit_index_io::load_index_from_path;
//use ragit_index_core::Index;
use ragit_utils::project_root::find_root;
use ragit_utils::doc_utils::get_doc_content;
//use ragit_index_io::index_struct::load_index_from_path;
use crate::prelude::Index;
use crate::prelude::load_index_from_path;


pub fn migrate_command_main(args: &[String]) -> Result<(), anyhow::Error> {
    let parsed_args = ArgParser::new().parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/migrate.txt"));
        return Ok(());
    }

    let root_dir = find_root()?;

    let mut index = load_index_from_path(&root_dir)?;
    if let Some((v1, v2)) = index.migrate(env!("CARGO_PKG_VERSION").to_string())? {
        println!("migrated from `{v1}` to `{v2}`");
    }

    let recover_result = index.recover()?;

    if !recover_result.is_empty() {
        println!("recovered from a corrupted knowledge-base: {recover_result:?}");
    }

    Ok(())
}
