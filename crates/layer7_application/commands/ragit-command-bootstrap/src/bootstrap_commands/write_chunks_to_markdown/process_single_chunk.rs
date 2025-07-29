use anyhow::Result;
use std::fmt::Write as FmtWrite;
use std::path::PathBuf;

use ragit_types::fixed_types::fixed_chunk_struct::FixedChunk;

use ragit_memory_monitor::MemoryMonitor;
use crate::bootstrap_commands::write_chunk_object;

pub async fn process_single_chunk(
    verbose: bool,
    temp_dir: &PathBuf,
    chunk: &FixedChunk,
    markdown_output: &mut String,
    processed_chunks_count: usize,
    total_chunk_files: usize,
    max_memory_gb: Option<u64>,
    memory_monitor: &mut MemoryMonitor,
    call_count: usize,
) -> Result<(), anyhow::Error> {
    if verbose { // Log every chunk
        println!("bootstrap_index_self: Processing chunk {}/{} (Call: {})", processed_chunks_count + 1, total_chunk_files, call_count);
        memory_monitor.capture_and_log_snapshot(&format!("Before loading chunk (chunk {}) (Call: {})", processed_chunks_count + 1, call_count));
    }
    memory_monitor.check_memory_limit(max_memory_gb, &format!("Before loading chunk (chunk {}) (Call: {})", processed_chunks_count + 1, call_count))?;

    let chunk_object_path = write_chunk_object::write_chunk_object(
        verbose,
        temp_dir,
        chunk,
        max_memory_gb,
        memory_monitor,
    ).await?;

    let relative_chunk_object_path = chunk_object_path.strip_prefix(temp_dir).unwrap().to_string_lossy().into_owned();

    if verbose {
        memory_monitor.capture_and_log_snapshot(&format!("After loading chunk (chunk {}) (Call: {})", processed_chunks_count + 1, call_count));
        println!("bootstrap_index_self: Appending chunk {} to markdown_output (Call: {})", processed_chunks_count + 1, call_count);
    }
    memory_monitor.check_memory_limit(max_memory_gb, &format!("After loading chunk (chunk {}) (Call: {})", processed_chunks_count + 1, call_count))?;

    writeln!(markdown_output, "Chunk UID: {}", chunk.uid)?;
    if verbose { println!("bootstrap_index_self: Appended UID: {}", chunk.uid); }

    writeln!(markdown_output, "Object Path: {}", relative_chunk_object_path)?;
    if verbose { println!("bootstrap_index_self: Appended Object Path: {}", relative_chunk_object_path); }

    writeln!(markdown_output, "Title: {}", chunk.title)?;
    if verbose { println!("bootstrap_index_self: Appended Title: {}", chunk.title); }

    writeln!(markdown_output, "Summary: {}", chunk.summary)?;
    if verbose { println!("bootstrap_index_self: Appended Summary: {}", chunk.summary); }

    writeln!(markdown_output, "Source: {:?}", chunk.source)?;
    if verbose { println!("bootstrap_index_self: Appended Source: {:?}", chunk.source); }

    writeln!(markdown_output, "Code Block: {}", chunk.data)?;
    if verbose { println!("bootstrap_index_self: Appended Code Block (length: {}): {}", chunk.data.len(), &chunk.data.as_str()[..std::cmp::min(chunk.data.len(), 50)]); }

    if verbose {
        memory_monitor.capture_and_log_snapshot(&format!("After appending chunk (chunk {}) (Call: {})", processed_chunks_count + 1, call_count));
    }
    memory_monitor.check_memory_limit(max_memory_gb, &format!("After appending chunk (chunk {}) (Call: {})", processed_chunks_count + 1, call_count))?;

    Ok(())
}
