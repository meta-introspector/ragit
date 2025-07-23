use crate::chunk::chunk_struct::Chunk;
use crate::prelude::*;
use std::io::Read;
use std::path::PathBuf;

pub fn load_from_file(path: &PathBuf) -> Result<Chunk> {
    let content = ragit_fs::read_bytes(path.to_str().unwrap())?;

    if content[0] == 0x1f && content[1] == 0x8b {
        let mut gz = flate2::read::GzDecoder::new(&content[1..]);
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
) -> Result<()> {
    let parent_path = ragit_fs::parent(path)?;

    if !ragit_fs::exists(&parent_path) {
        ragit_fs::try_create_dir(parent_path.to_str().unwrap())?;
    }

    if create_tfidf {
        let tfidf_path = PathBuf::from(ragit_fs::set_extension(path.to_str().unwrap(), "tfidf")?);
        crate::index::tfidf::save_to_file(
            tfidf_path.to_str().unwrap(),
            chunk,
            root_dir.to_str().unwrap(),
        )?;
    }

    let serialized_chunk = serde_json::to_vec(chunk)?;

    if serialized_chunk.len() as u64 > compression_threshold {
        use std::io::Write;
        let mut encoder =
            flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::new(compression_level));
        encoder.write_all(&serialized_chunk)?;
        let compressed_bytes = encoder.finish()?;
        Ok(ragit_fs::write_bytes(
            path.to_str().unwrap(),
            &compressed_bytes,
            ragit_fs::WriteMode::CreateOrTruncate,
        )?)
    } else {
        Ok(ragit_fs::write_bytes(
            path.to_str().unwrap(),
            &serialized_chunk,
            ragit_fs::WriteMode::CreateOrTruncate,
        )?)
    }
}
