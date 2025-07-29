use anyhow::Result;
use std::path::PathBuf;
use std::fs;
use sha3::{Digest, Sha3_256};
use ragit_types::fixed_types::fixed_chunk_struct::FixedChunk;
use ragit_memory_monitor::MemoryMonitor;

pub async fn write_chunk_object(
    verbose: bool,
    temp_dir: &PathBuf,
    chunk: &FixedChunk,
    max_memory_gb: Option<u64>,
    memory_monitor: &mut MemoryMonitor,
) -> Result<PathBuf, anyhow::Error> {
    if verbose { println!("DEBUG: Entering write_chunk_object for chunk with UID: {}", chunk.uid); }
    let mut hasher = Sha3_256::new();
    hasher.update(chunk.data.as_str().as_bytes());
    let hash_bytes = hasher.finalize();
    let hash_string = format!("{:x}", hash_bytes);

    let object_dir = temp_dir.join(".ragit").join("objects");
    if verbose { println!("DEBUG: Attempting to create object_dir: {:?}", object_dir); }
    fs::create_dir_all(&object_dir)?;
    if verbose { println!("DEBUG: Successfully created object_dir: {:?}", object_dir); }
    let first_two_chars = &hash_string[0..2];
    let rest_of_hash = &hash_string[2..];

    let chunk_dir = object_dir.join(first_two_chars);
    if verbose { println!("DEBUG: Attempting to create chunk_dir: {:?}", chunk_dir); }
    fs::create_dir_all(&chunk_dir)?;
    if verbose { println!("DEBUG: Successfully created chunk_dir: {:?}", chunk_dir); }

    let chunk_path = chunk_dir.join(rest_of_hash);
    if verbose { println!("DEBUG: Attempting to write chunk to: {:?}", chunk_path); }
    fs::write(&chunk_path, chunk.data.as_str().as_bytes())?;
    if verbose { println!("DEBUG: Successfully wrote chunk to: {:?}", chunk_path); }

    if verbose {
        println!("bootstrap_index_self: Wrote chunk to content-addressable store: {:?}", chunk_path);
        memory_monitor.capture_and_log_snapshot(&format!("Wrote chunk object: {:?}", chunk_path));
    }
    memory_monitor.check_memory_limit(max_memory_gb, &format!("After writing chunk object: {:?}", chunk_path))?;

    Ok(chunk_path)
}
