use anyhow::Result;
use std::fmt::Write as FmtWrite;
use std::path::PathBuf;
use sysinfo::System;
use ragit_index_io::StorageManager;
use ragit_index_types::index_struct::Index;
use ragit_fs::{write_string, WriteMode};
use super::constants::{CHUNK_PROCESSING_LIMIT, CHUNKS_OUTPUT_FILE_NAME};
use crate::bootstrap_commands::memory_utils::print_memory_usage;

pub async fn write_chunks_to_markdown(
    verbose: bool,
    temp_dir: &PathBuf,
    index: &Index,
    sys: &mut System,
) -> Result<(), anyhow::Error> {
    if verbose {
        println!("bootstrap_index_self: Writing chunks to markdown file");
        println!("bootstrap_index_self: Before storage_manager.load_all_chunks");
        print_memory_usage(sys, "Before load_all_chunks");
    }
    let storage_manager = StorageManager::new(index.root_dir.clone());
    let all_chunks = storage_manager.load_all_chunks().await?;
    let all_chunks_len = all_chunks.len();
    if verbose {
        println!("bootstrap_index_self: After storage_manager.load_all_chunks");
        print_memory_usage(sys, "After load_all_chunks");
    }
    let mut markdown_output = String::new();
    for (i, chunk) in all_chunks.into_iter().enumerate() {
        if i >= CHUNK_PROCESSING_LIMIT { // Stop after CHUNK_PROCESSING_LIMIT chunks
            println!("bootstrap_index_self: Stopping chunk processing after {} chunks.", CHUNK_PROCESSING_LIMIT);
            break;
        }
        if verbose { // Log every chunk
            println!("bootstrap_index_self: Processing chunk {}/{}", i, all_chunks_len);
            print_memory_usage(sys, &format!("During chunk processing (chunk {})", i));
        }
        writeln!(&mut markdown_output, "Chunk UID: {}", chunk.uid)?;
        writeln!(&mut markdown_output, "Title: {}", chunk.title)?;
        writeln!(&mut markdown_output, "Summary: {}", chunk.summary)?;
        writeln!(&mut markdown_output, "Source: {:?}", chunk.source)?;
        writeln!(&mut markdown_output, "Code Block: {}", chunk.data)?;
    }
    if verbose {
        print_memory_usage(sys, "After chunk processing loop");
    }
    let chunks_file_path = temp_dir.join(CHUNKS_OUTPUT_FILE_NAME);
    write_string(chunks_file_path.to_str().unwrap(), &markdown_output, WriteMode::CreateOrTruncate)?;
    if verbose {
        println!("bootstrap_index_self: Chunks written to {:?}", chunks_file_path);
    }
    Ok(())
}
