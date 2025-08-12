use anyhow::Result;
use ragit_types::Chunk;
use ragit_types::ApiError;
use std::path::PathBuf;
use ragit_index_storage;
use ragit_tfidf;
use ragit_utils::constant::CHUNK_DIR_NAME;

pub struct StorageManager {
    root_dir: PathBuf,
}

impl StorageManager {
    pub fn new(root_dir: PathBuf) -> Self {
        Self { root_dir }
    }

    pub async fn load_all_chunks(&self) -> Result<Vec<Chunk>, ApiError> {
        let mut chunks = vec![];

        let chunk_file_paths = ragit_index_storage::get_files_from_index_subdir(&self.root_dir, CHUNK_DIR_NAME, Some("chunk"))?;

        for chunk_path in &chunk_file_paths {
            eprintln!("Loading chunk from: {:?}", chunk_path);
            chunks.push(Chunk::from(ragit_tfidf::io::load_from_file(chunk_path.to_str().unwrap())?));
        }

        Ok(chunks)
    }
}
