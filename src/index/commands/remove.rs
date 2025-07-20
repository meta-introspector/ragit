use crate::index::index_struct::Index;
use crate::error::Error;
use crate::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RemoveResult {
    pub success: usize,
    pub errors: usize,
}

impl Index {
    pub fn remove(
        &mut self,
        file: Path,
        dry_run: bool,
        recursive: bool,
        auto: bool,
        staged: bool,
        processed: bool,
    ) -> Result<RemoveResult, Error> {
        // TODO: Implement dry_run, recursive, auto, staged, processed logic
        // For now, just focus on the core removal logic and return type.

        let real_path = Index::get_data_path(&self.root_dir, &file)?;
        let file_uid = crate::uid::Uid::new_file(self.root_dir.to_str().unwrap(), real_path.to_str().unwrap())?;

        let result = self.remove_file_index(file_uid);

        match result {
            Ok(_) => Ok(RemoveResult { success: 1, errors: 0 }),
            Err(e) => {
                eprintln!("Error removing file index: {:?}", e);
                Ok(RemoveResult { success: 0, errors: 1 })
            }
        }
    }
}
