pub fn gc_command_main(args: &[String]) -> Result<(), anyhow::Error> {
    panic!("FIXME: ragit-command-gc not implemented");
}

/*
use ragit_utils::prelude::*;
use ragit_api::prelude::*;
use ragit_types::prelude::*;
use ragit_index_core::load_index_from_path;
use ragit_index_types::index_struct::Index;

use ragit_utils::project_root::find_root;
use ragit_utils::doc_utils::get_doc_content;

pub fn gc_command_main(args: &[String]) -> Result<(), anyhow::Error> {
    let parsed_args = ArgParser::new()
        .flag(&["--logs", "--images", "--audit", "--all"])
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/gc.txt"));
        return Ok(());
    }

    let mut index = load_index_from_path(&find_root()?)?;

    match parsed_args.get_flag(0).unwrap().as_str() {
        "--logs" => {
            let removed = index.gc_logs()?;
            println!("removed {removed} log files");
        }
        "--images" => {
            let removed = index.gc_images()?;
            println!("removed {removed} images");
        }
        "--audit" => {
            index.gc_audit()?;
            println!("removed audit logs");
        }
        "--all" => {
            let removed_logs = index.gc_logs()?;
            let removed_images = index.gc_images()?;
            index.gc_audit()?;
            println!("removed {removed_logs} log files, {removed_images} images and audit logs");
        }
        _ => unreachable!(),
    }

    Ok(())
}
*/
