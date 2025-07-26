use ragit_utils::prelude::*;
use ragit_utils::doc_utils::get_doc_content;
use ragit_index_types::index_struct::Index;
use ragit_utils::project_root::find_root;
use std::path::Path;

pub async fn init_command_main(args: &[String], root_dir_override: Option<&Path>) -> Result<(), anyhow::Error> {
    let parsed_args = ArgParser::new().parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/init.txt"));
        return Ok(());
    }

    let root = if let Some(path) = root_dir_override {
        path.to_path_buf()
    } else {
        find_root()? 
    };
    let index = Index::new(root);
    // TODO: Save the index to disk
    println!("initialized");

    Ok(())
}
