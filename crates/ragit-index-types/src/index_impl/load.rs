use crate::index_struct::Index;
use ragit_error::ApiError;
use std::path::PathBuf;
use crate::load_mode::LoadMode;

impl Index {
    pub fn load(path: PathBuf, mode: LoadMode) -> Result<Self, ApiError> {
        eprintln!("Placeholder for load: path={:?}, mode={:?}", path, mode);
        Ok(Index::new(path))
    }
}