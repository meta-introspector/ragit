use super::prelude::*;
use crate::index_struct::Index;
use ragit_error::ApiError;
use ragit_types::Uid;
impl Index {
    pub fn reset_uid(&mut self, save_to_file: bool) -> Result<(), ApiError> {
        self.uid = Uid::dummy();

        if save_to_file {
            ragit_index_save_to_file::save_index_to_file(self, self.root_dir.join(INDEX_FILE_NAME))?;
        }

        Ok(())
    }
}
