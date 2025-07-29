use anyhow::Result;
use ragit_index_types::index_impl::set_config_by_key::index_set_config_by_key;
use ragit_index_types::index_struct::Index;
use std::io::Write;
use ragit_memory_monitor::MemoryMonitor;

pub async fn configure_memory_settings(
    verbose: bool,
    index: &mut Index,
    max_memory_gb: Option<u64>,
    memory_monitor: &mut MemoryMonitor,
) -> Result<()> {
    if verbose {
        println!("configure_memory_settings: Starting");
        memory_monitor.capture_and_log_snapshot("Before configuring memory settings");
    }

    memory_monitor.check_memory_limit(max_memory_gb, "Before setting max_chunk_size")?;
    index_set_config_by_key(index, "max_chunk_size".to_string(), "256".to_string())?;
    if verbose {
        writeln!(std::io::stdout(), "Set max_chunk_size to 256")?;
    }

    memory_monitor.check_memory_limit(max_memory_gb, "Before setting max_summary_len")?;
    index_set_config_by_key(index, "max_summary_len".to_string(), "500".to_string())?;
    if verbose {
        writeln!(std::io::stdout(), "Set max_summary_len to 500")?;
    }

    memory_monitor.check_memory_limit(max_memory_gb, "Before setting min_summary_len")?;
    index_set_config_by_key(index, "min_summary_len".to_string(), "100".to_string())?;
    if verbose {
        writeln!(std::io::stdout(), "Set min_summary_len to 100")?;
    }

    // Note: max_summaries and max_retrieval are query_config settings, not build_config.
    // They are not directly set via index_set_config_by_key in the current implementation
    // as seen in set_config_by_key.rs. We will skip these for now to avoid errors.
    // If these need to be set, the index_set_config_by_key function or the Index struct
    // would need to be updated to expose these as configurable via this method.

    if verbose {
        println!("configure_memory_settings: Finished");
        memory_monitor.capture_and_log_snapshot("After configuring memory settings");
    }

    Ok(())
}
