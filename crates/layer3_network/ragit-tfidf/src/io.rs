//use ragit_types::ApiError;
use ragit_types::processed_doc::ProcessedDoc;
use ragit_fs::{read_bytes, write_bytes, WriteMode};
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use ragit_types::chunk::chunk_struct::Chunk;
use anyhow::Result;
use ragit_types::ApiError;
use std::sync::Arc;
pub fn load_from_file(path: &str) -> Result<ProcessedDoc, ApiError> {
    let content = read_bytes(path)?;
    let mut gz = GzDecoder::new(&content[..]);
    let mut s = String::new();
    std::io::Read::read_to_string(&mut gz, &mut s).map_err(|e| ApiError::StdIoError(Arc::new(e)))?;
    Ok(serde_json::from_str(&s).map_err(|e| ApiError::JsonSerdeError(Arc::new(e)))?)
}

pub fn save_to_file(path: &str, chunk: &Chunk, _root_dir: &str) -> Result<(), ApiError> {
    let mut gz = GzEncoder::new(Vec::new(), Compression::best());
    std::io::Write::write_all(&mut gz, &serde_json::to_string(chunk).map_err(|e| ApiError::JsonSerdeError(Arc::new(e)))?.as_bytes()).map_err(|e| ApiError::StdIoError(Arc::new(e)))?;
    let result = gz.finish().unwrap();
    write_bytes(path, &result, WriteMode::CreateOrTruncate)?;
    Ok(())
}
