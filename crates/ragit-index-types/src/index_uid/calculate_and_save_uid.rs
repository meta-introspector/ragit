use super::prelude::*;
use crate::index_struct::Index;
use ragit_error::ApiError;
use ragit_types::Uid;
impl Index {
    pub fn calculate_and_save_uid(&mut self) -> Result<Uid, ApiError> {
        let uid = self.calculate_uid(false)?;
        self.uid = uid;
        ragit_index_save_to_file::save_index_to_file(self, self.root_dir.join(INDEX_FILE_NAME))?;
        Ok(uid)
    }
}
