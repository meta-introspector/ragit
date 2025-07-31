use anyhow::Result;
use std::path::PathBuf;
use std::fs;
use sha3::{Digest, Sha3_256};
use ragit_types::fixed_types::fixed_chunk_struct::FixedChunk;
use ragit_memory_monitor::MemoryMonitor;

pub async fn write_chunk_object(
    _verbose: bool,
    temp_dir: &PathBuf,
    chunk: &FixedChunk,
    max_memory_gb: Option<u64>,
    memory_monitor: &mut MemoryMonitor,
) -> Result<PathBuf, anyhow::Error> {
    memory_monitor.verbose(&format!("write_chunk_object: Entering for chunk with UID: {}", chunk.uid));
    let mut hasher = Sha3_256::new();
    hasher.update(chunk.data.as_str().as_bytes());
    let hash_bytes = hasher.finalize();
    let hash_string = format!("{:x}", hash_bytes);

    let object_dir = temp_dir.join(".ragit").join("objects");
    memory_monitor.verbose(&format!("write_chunk_object: Attempting to create object_dir: {:?}", object_dir));
    fs::create_dir_all(&object_dir)?;
    memory_monitor.verbose(&format!("write_chunk_object: Successfully created object_dir: {:?}", object_dir));
    let first_two_chars = &hash_string[0..2];
    let rest_of_hash = &hash_string[2..];

    let chunk_dir = object_dir.join(first_two_chars);
    memory_monitor.verbose(&format!("write_chunk_object: Attempting to create chunk_dir: {:?}", chunk_dir));
    fs::create_dir_all(&chunk_dir)?;
    memory_monitor.verbose(&format!("write_chunk_object: Successfully created chunk_dir: {:?}", chunk_dir));

    let chunk_path = chunk_dir.join(rest_of_hash);
    memory_monitor.verbose(&format!("write_chunk_object: Attempting to write chunk to: {:?}", chunk_path));
    fs::write(&chunk_path, chunk.data.as_str().as_bytes())?;
    memory_monitor.verbose(&format!("write_chunk_object: Successfully wrote chunk to: {:?}", chunk_path));

    memory_monitor.verbose(&format!("write_chunk_object: Wrote chunk to content-addressable store: {:?}", chunk_path));
    memory_monitor.capture_and_log_snapshot(&format!("Wrote chunk object: {:?}", chunk_path));
    memory_monitor.check_memory_limit(max_memory_gb, &format!("After writing chunk object: {:?}", chunk_path))?;
    memory_monitor.verbose("write_chunk_object: Exiting.");
    Ok(chunk_path)
}
