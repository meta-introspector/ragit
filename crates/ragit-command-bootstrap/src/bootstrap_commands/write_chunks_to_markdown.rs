use anyhow::Result;
use std::fmt::Write as FmtWrite;
use std::path::PathBuf;
use sysinfo::System;
use ragit_index_types::index_struct::Index;
use ragit_fs::{write_string, WriteMode};
use ragit_index_storage;
use ragit_tfidf;
use ragit_types::Chunk;
use ragit_tfidf;
use ragit_types::Chunk;
use super::constants::{CHUNK_PROCESSING_LIMIT, CHUNKS_OUTPUT_FILE_NAME};
use crate::bootstrap_commands::memory_utils::{print_memory_usage, check_memory_limit};

pub async fn write_chunks_to_markdown(
    verbose: bool,
    temp_dir: &PathBuf,
    index: &Index,
    sys: &mut System,
    max_memory_gb: Option<u64>,
    last_process_memory_kb: &mut Option<u64>,
) -> Result<(), anyhow::Error> {
    if verbose {
        println!("bootstrap_index_self: Writing chunks to markdown file");
        print_memory_usage(sys, "Before iterating chunk files", last_process_memory_kb);
    }
    check_memory_limit(sys, max_memory_gb, "Before iterating chunk files")?;

    let mut markdown_output = String::new();
    let mut processed_chunks_count = 0;

    let all_chunk_files = ragit_index_storage::get_all_chunk_files(&index.root_dir)?;
    let total_chunk_files = all_chunk_files.len();

    for chunk_path in all_chunk_files {
        if processed_chunks_count >= CHUNK_PROCESSING_LIMIT { // Stop after CHUNK_PROCESSING_LIMIT chunks
            println!("bootstrap_index_self: Stopping chunk processing after {} chunks.", CHUNK_PROCESSING_LIMIT);
            break;
        }

        if verbose { // Log every chunk
            println!("bootstrap_index_self: Processing chunk {}/{}", processed_chunks_count, total_chunk_files);
            print_memory_usage(sys, &format!("During chunk processing (chunk {})", processed_chunks_count), last_process_memory_kb);
        }
        check_memory_limit(sys, max_memory_gb, &format!("During chunk processing (chunk {})", processed_chunks_count))?;

        let chunk = Chunk::from(ragit_tfidf::io::load_from_file(chunk_path.to_str().unwrap())?);

        writeln!(&mut markdown_output, "Chunk UID: {}", chunk.uid)?;
        writeln!(&mut markdown_output, "Title: {}", chunk.title)?;
        writeln!(&mut markdown_output, "Summary: {}", chunk.summary)?;
        writeln!(&mut markdown_output, "Source: {:?}", chunk.source)?;
        writeln!(&mut markdown_output, "Code Block: {}", chunk.data)?;

        processed_chunks_count += 1;
    }

    if verbose {
        print_memory_usage(sys, "After chunk processing loop", last_process_memory_kb);
    }
    check_memory_limit(sys, max_memory_gb, "After chunk processing loop")?;

    let chunks_file_path = temp_dir.join(CHUNKS_OUTPUT_FILE_NAME);
    write_string(chunks_file_path.to_str().unwrap(), &markdown_output, WriteMode::CreateOrTruncate)?;
    if verbose {
        println!("bootstrap_index_self: Chunks written to {:?}", chunks_file_path);
    }
    Ok(())
}
