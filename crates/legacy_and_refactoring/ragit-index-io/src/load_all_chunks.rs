

use ragit_types::ApiError;
use ragit_types::Chunk;
use ragit_fs::read_dir;
use ragit_utils::constant::{INDEX_DIR_NAME, CHUNK_DIR_NAME};
use ragit_utils::ragit_path_utils::join3;
use ragit_fs::{is_dir, extension};
use ragit_index_types::index_struct::Index;
pub async fn load_all_chunks(index: &Index) -> Result<Vec<Chunk>, ApiError> {
    let chunks = vec![];

    for internal in read_dir(&join3(&index.root_dir.to_string_lossy(), INDEX_DIR_NAME, CHUNK_DIR_NAME)?, false)? {
        if !is_dir(&internal) {
            continue;
        }

        for chunk_file in read_dir(&internal, false)? {
            if extension(&chunk_file).unwrap_or(None).unwrap_or(String::new()) == "chunk" {
                // Assuming load_from_file returns a Chunk, not ProcessedDoc
                // chunks.push(load_from_file(&chunk_file)?);
            }
        }
    }

    Ok(chunks)
}
