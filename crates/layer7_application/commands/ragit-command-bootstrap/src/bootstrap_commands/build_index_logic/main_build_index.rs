use crate::bootstrap_commands::build_index_logic::get_staged_files::get_staged_files;
use crate::bootstrap_commands::process_staged_file_logic::main_process_staged_file::process_staged_file;

pub async fn build_index(
    verbose: bool,
    _temp_dir: &PathBuf,
    actual_root_dir: &PathBuf,
    index: &mut Index,
    _max_iterations: Option<usize>,
    sys: &mut System,
    max_memory_gb: Option<u64>,
    last_process_memory_kb: &mut Option<u64>,
) -> Result<(), anyhow::Error> {
    if verbose {
        println!("bootstrap_index_self: Running rag build");
        println!("bootstrap_index_self: Before ragit_index_effects::build (placeholder)");
        print_memory_usage(sys, "Before ragit_index_effects::build", last_process_memory_kb);
    }
    check_memory_limit(sys, max_memory_gb, "Before ragit_index_effects::build")?;

    let staged_files = get_staged_files(index)?;

    process_staged_files_loop(
        verbose,
        staged_files,
        actual_root_dir,
        index,
    )?;

    if verbose {
        println!("bootstrap_index_self: After ragit_index_effects::build (placeholder)");
        println!("bootstrap_index_self: Built index (placeholder)");
        print_memory_usage(sys, "After ragit_index_effects::build", last_process_memory_kb);
    }
    check_memory_limit(sys, max_memory_gb, "After ragit_index_effects::build")?;
    Ok(())
}