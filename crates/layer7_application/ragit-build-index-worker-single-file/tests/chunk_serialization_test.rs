use anyhow::Result;
use std::env;
use std::path::PathBuf;
use ragit_types::fixed_types::fixed_chunk_struct::FixedChunk;
use ragit_types::chunk::ChunkSource;
use ragit_types::Uid;
use ragit_types::fixed_types::fixed_chunk_struct::FixedChunkBuildInfo;
use chrono;
use serde_json;
use std::fs;
use std::io::Write;

#[test]
fn test_fixed_chunk_serialization_size() -> Result<()> {
    let dummy_chunk = FixedChunk {
        data: "This is some dummy data for the chunk.".into(),
        images: Default::default(),
        char_len: 0,
        image_count: 0,
        title: "Dummy Chunk Title".into(),
        summary: "This is a summary of the dummy chunk.".into(),
        file: "/path/to/dummy/file.txt".into(),
        index: 0,
        source: ChunkSource::File { path: "/path/to/dummy/file.txt".to_string(), index: 0, page: Some(0) },
        uid: Uid::dummy(),
        build_info: FixedChunkBuildInfo::default(),
        timestamp: chrono::Utc::now().timestamp(),
        searchable: true,
    };

    // Serialize the chunk to JSON
    let serialized_chunk = serde_json::to_string_pretty(&dummy_chunk)?;

    // Write to a temporary file
    let temp_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target").join("debug");
    fs::create_dir_all(&temp_dir)?;
    let temp_file_path = temp_dir.join("temp_dummy_chunk.json");
    let mut file = fs::File::create(&temp_file_path)?;
    file.write_all(serialized_chunk.as_bytes())?;

    // Get the size of the file
    let metadata = fs::metadata(&temp_file_path)?;
    let file_size = metadata.len();

    println!("Serialized dummy chunk size: {} bytes", file_size);

    // Deserialize to verify
    let deserialized_chunk: FixedChunk = serde_json::from_str(&serialized_chunk)?;
    assert_eq!(dummy_chunk, deserialized_chunk);

    // Keep the temporary file for inspection
    println!("Temporary dummy chunk file saved at: {}", temp_file_path.display());

    Ok(())
}