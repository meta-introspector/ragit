use anyhow::Result;
use std::path::PathBuf;
use ragit_memory_monitor::MemoryMonitor;
use ragit_index_types::index_struct::Index;
use crate::bootstrap_commands::write_chunk_object;

pub async fn write_chunks_to_markdown(
    verbose: bool,
    temp_dir: &PathBuf,
    index: &Index,
    max_memory_gb: Option<u64>,
    memory_monitor: &mut MemoryMonitor,
    max_iterations: Option<usize>,
) -> Result<(), anyhow::Error> {
    memory_monitor.verbose("Writing chunks to content-addressable objects.");

    let all_chunks = index.get_chunks();
    memory_monitor.verbose(&format!("Number of chunks returned by index.get_chunks(): {}", all_chunks.len()));
    memory_monitor.verbose(&format!("Number of chunks to process: {}", all_chunks.len()));
    let mut processed_chunks_count = 0;

    for chunk in all_chunks {
        memory_monitor.verbose(&format!("Processing chunk. temp_dir: {:?}", temp_dir));
        if let Some(max_iter) = max_iterations {
            if processed_chunks_count >= max_iter {
                memory_monitor.verbose(&format!("Stopping chunk processing after {} chunks due to max_iterations limit.", max_iter));
                break;
            }
        }

        memory_monitor.verbose(&format!("Calling write_chunk_object for chunk with UID: {}", chunk.uid));
        write_chunk_object::write_chunk_object(
            verbose,
            temp_dir,
            &chunk,
            max_memory_gb,
            memory_monitor,
        ).await?;

        processed_chunks_count += 1;
    }

    memory_monitor.verbose(&format!("Finished writing {} chunks to content-addressable objects.", processed_chunks_count));

    Ok(())
}