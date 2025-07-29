use anyhow::Result;
use std::path::PathBuf;

use ragit_index_types::index_struct::Index;
use ragit_types::build_config::BuildConfig;
// use crate::bootstrap_commands::process_staged_file_logic::main_process_staged_file::process_staged_file;

pub fn process_staged_files_loop(
    verbose: bool,
    staged_files: Vec<PathBuf>,
    actual_root_dir: &PathBuf,
    index: &mut Index,
) -> Result<(), anyhow::Error> {
    if verbose {
        println!("bootstrap_index_self: Iterating through staged files for chunking and indexing.");
    }

    for file_path_buf in staged_files {
        // process_staged_file(
        //     verbose,
        //     &file_path_buf,
        //     actual_root_dir,
        //     &index.build_config,
        //     index,
        // )?;
    }
    Ok(())
}
