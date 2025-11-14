use anyhow::Result;
use ragit_index_types::index_impl::set_config_by_key::index_set_config_by_key;
use ragit_index_types::index_struct::Index;

use ragit_memory_monitor::MemoryMonitor;

pub async fn configure_memory_settings(
    verbose: bool,
    index: &mut Index,
    max_memory_gb: Option<u64>,
    max_chunk_size: Option<usize>,
    max_summary_len: Option<usize>,
    min_summary_len: Option<usize>,
    memory_monitor: &mut MemoryMonitor,
) -> Result<()> {
    memory_monitor.verbose("Configuring memory settings.");

    let actual_max_chunk_size = max_chunk_size.unwrap_or(256);
    memory_monitor.check_memory_limit(max_memory_gb, &format!("Before setting max_chunk_size to {}", actual_max_chunk_size))?;
    index_set_config_by_key(index, "max_chunk_size".to_string(), actual_max_chunk_size.to_string())?;
    memory_monitor.verbose(&format!("Set max_chunk_size to {}", actual_max_chunk_size));

    let actual_max_summary_len = max_summary_len.unwrap_or(500);
    memory_monitor.check_memory_limit(max_memory_gb, &format!("Before setting max_summary_len to {}", actual_max_summary_len))?;
    index_set_config_by_key(index, "max_summary_len".to_string(), actual_max_summary_len.to_string())?;
    memory_monitor.verbose(&format!("Set max_summary_len to {}", actual_max_summary_len));

    let actual_min_summary_len = min_summary_len.unwrap_or(100);
    memory_monitor.check_memory_limit(max_memory_gb, &format!("Before setting min_summary_len to {}", actual_min_summary_len))?;
    index_set_config_by_key(index, "min_summary_len".to_string(), actual_min_summary_len.to_string())?;
    memory_monitor.verbose(&format!("Set min_summary_len to {}", actual_min_summary_len));

    // Note: max_summaries and max_retrieval are query_config settings, not build_config.
    // They are not directly set via index_set_config_by_key in the current implementation
    // as seen in set_config_by_key.rs. We will skip these for now to avoid errors.
    // If these need to be set, the index_set_config_by_key function or the Index struct
    // would need to be updated to expose these as configurable via this method.

    if verbose {
        memory_monitor.verbose("configure_memory_settings: Finished");
        memory_monitor.capture_and_log_snapshot("After configuring memory settings");
    }

    Ok(())
}
