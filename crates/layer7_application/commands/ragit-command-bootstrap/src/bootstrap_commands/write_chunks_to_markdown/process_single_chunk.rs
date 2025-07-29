use anyhow::Result;
use std::fmt::Write as FmtWrite;
use sysinfo::System;
use ragit_types::fixed_types::fixed_chunk_struct::FixedChunk;
use crate::bootstrap_commands::memory_utils::{print_memory_usage, check_memory_limit};

pub async fn process_single_chunk(
    verbose: bool,
    chunk: &FixedChunk,
    markdown_output: &mut String,
    processed_chunks_count: usize,
    total_chunk_files: usize,
    sys: &mut System,
    max_memory_gb: Option<u64>,
    last_process_memory_kb: &mut Option<u64>,
    call_count: usize,
) -> Result<(), anyhow::Error> {
    if verbose { // Log every chunk
        println!("bootstrap_index_self: Processing chunk {}/{} (Call: {})", processed_chunks_count + 1, total_chunk_files, call_count);
        print_memory_usage(sys, &format!("Before loading chunk (chunk {}) (Call: {})", processed_chunks_count + 1, call_count), last_process_memory_kb);
    }
    check_memory_limit(sys, max_memory_gb, &format!("Before loading chunk (chunk {}) (Call: {})", processed_chunks_count + 1, call_count))?;

    if verbose {
        print_memory_usage(sys, &format!("After loading chunk (chunk {}) (Call: {})", processed_chunks_count + 1, call_count), last_process_memory_kb);
        println!("bootstrap_index_self: Appending chunk {} to markdown_output (Call: {})", processed_chunks_count + 1, call_count);
    }
    check_memory_limit(sys, max_memory_gb, &format!("After loading chunk (chunk {}) (Call: {})", processed_chunks_count + 1, call_count))?;

    writeln!(markdown_output, "Chunk UID: {}", chunk.uid)?;
    if verbose { println!("bootstrap_index_self: Appended UID: {}", chunk.uid); }

    writeln!(markdown_output, "Title: {}", chunk.title)?;
    if verbose { println!("bootstrap_index_self: Appended Title: {}", chunk.title); }

    writeln!(markdown_output, "Summary: {}", chunk.summary)?;
    if verbose { println!("bootstrap_index_self: Appended Summary: {}", chunk.summary); }

    writeln!(markdown_output, "Source: {:?}", chunk.source)?;
    if verbose { println!("bootstrap_index_self: Appended Source: {:?}", chunk.source); }

    writeln!(markdown_output, "Code Block: {}", chunk.data)?;
    if verbose { println!("bootstrap_index_self: Appended Code Block (length: {}): {}", chunk.data.len(), &chunk.data.as_str()[..std::cmp::min(chunk.data.len(), 50)]); }

    if verbose {
        print_memory_usage(sys, &format!("After appending chunk (chunk {}) (Call: {})", processed_chunks_count + 1, call_count), last_process_memory_kb);
    }
    check_memory_limit(sys, max_memory_gb, &format!("After appending chunk (chunk {}) (Call: {})", processed_chunks_count + 1, call_count))?;

    Ok(())
}
