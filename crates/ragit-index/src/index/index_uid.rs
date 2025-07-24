use crate::prelude::*;

impl Index {
    pub fn calculate_and_save_uid(&mut self) -> Result<Uid, ApiError> {
        let uid = self.calculate_uid(false)?;
        self.uid = uid;
        self.save_to_file(self.root_dir.join(INDEX_FILE_NAME).to_str().unwrap())?;
        Ok(uid)
    }

    pub fn calculate_uid(&self, force: bool) -> Result<Uid, ApiError> {
        if !force && self.uid != Uid::new() {
            return Ok(self.uid);
        }

        let mut uids = vec![];

        for file in self.processed_files.keys() {
            let file_index_path = get_uid_path(&self.root_dir, FILE_INDEX_DIR_NAME, Uid::new(), Some(file.to_str().unwrap()))?;

            if exists(&file_index_path) {
                for uid in load_from_file(&file_index_path)? {
                    uids.push(uid);
                }
            }
        }

        uids.sort();

        Ok(Uid::new_from_uids(&uids))
    }

    pub fn reset_uid(&mut self, save_to_file: bool) -> Result<(), ApiError> {
        self.uid = Uid::new();

        if save_to_file {
            self.save_to_file(self.root_dir.join(INDEX_FILE_NAME).to_str().unwrap())?;
        }

        Ok(())
    }
}