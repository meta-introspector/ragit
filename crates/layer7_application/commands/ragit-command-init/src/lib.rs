use ragit_utils::prelude::*;
use ragit_utils::doc_utils::get_doc_content;
use ragit_index_types::index_struct::Index;
use ragit_utils::project_root::find_root;
use std::path::Path;
use std::fs;
use ragit_index_save_to_file::save_index_to_file;

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

    let ragit_dir = root.join(".ragit");
    if !ragit_dir.exists() {
        fs::create_dir_all(&ragit_dir)?;
    }

    let index = Index::new(root.clone());
    let index_path = ragit_dir.join("index.json");
    save_index_to_file(&index, index_path)?;
    
    println!("initialized");

    Ok(())
}
