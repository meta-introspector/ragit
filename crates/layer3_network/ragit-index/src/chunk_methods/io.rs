use std::io::Read;
use std::path::PathBuf;
use ragit_types::Chunk;
use anyhow::Error;
use ragit_fs::{read_bytes, parent, exists, try_create_dir, set_extension, write_bytes, WriteMode};
use flate2::read::GzDecoder;
use flate2::{Compression, write::GzEncoder};
use serde_json;
use ragit_tfidf;

pub fn load_from_file(path: &PathBuf) -> Result<Chunk, Error> {
    let content = read_bytes(path.to_str().unwrap())?;

    if content[0] == 0x1f && content[1] == 0x8b {
        let mut gz = GzDecoder::new(&content[1..]);
        let mut s = String::new();
        gz.read_to_string(&mut s)?;
        Ok(serde_json::from_str(&s)?)
    } else {
        Ok(serde_json::from_slice(&content)?)
    }
}

pub fn save_to_file(
    path: &PathBuf,
    chunk: &Chunk,
    compression_threshold: u64,
    compression_level: u32,
    root_dir: &PathBuf,
    create_tfidf: bool,
) -> Result<(), Error> {
    let parent_path = parent(path)?;

    if !exists(&parent_path) {
        try_create_dir(parent_path.to_str().unwrap())?;
    }

    if create_tfidf {
        let tfidf_path = PathBuf::from(set_extension(path.to_str().unwrap(), "tfidf")?);
        ragit_tfidf::save_to_file(
            tfidf_path.to_str().unwrap(),
            chunk,
            root_dir.to_str().unwrap(),
        )?;
    }

    let serialized_chunk = serde_json::to_vec(chunk)?;

    if serialized_chunk.len() as u64 > compression_threshold {
        use std::io::Write;
        let mut encoder =
            GzEncoder::new(Vec::new(), Compression::new(compression_level));
        encoder.write_all(&serialized_chunk)?;
        let compressed_bytes = encoder.finish()?;
        Ok(write_bytes(
            path.to_str().unwrap(),
            &compressed_bytes,
            WriteMode::CreateOrTruncate,
        )?)
    } else {
        Ok(write_bytes(
            path.to_str().unwrap(),
            &serialized_chunk,
            WriteMode::CreateOrTruncate,
        )?)
    }
}