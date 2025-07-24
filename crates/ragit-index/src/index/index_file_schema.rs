
use crate::prelude::*;

impl Index {
    pub fn get_file_schema(
        &self,
        path: Option<String>,
        uid: Option<Uid>,
    ) -> Result<FileSchema, Error> {
        if let Some(uid) = uid {
            for (path, uid_) in self.processed_files.iter() {
                if uid == *uid_ {
                    return self.get_file_schema_worker(path.display().to_string(), uid);
                }
            }
        }

        if let Some(path) = &path {
            if let Some(uid) = self.processed_files.get(Path::new(path)) {
                return self.get_file_schema_worker(path.to_string(), *uid);
            }

            if self.staged_files.contains(&PathBuf::from(path)) {
                return Ok(FileSchema {
                    path: path.to_string(),
                    is_processed: false,
                    ..FileSchema::dummy()
                });
            }
        }

        Err(Error::NoSuchFile {
            path: path.map(|s| s.into()),
            uid,
        })
    }

    pub(crate) fn get_file_schema_worker(
        &self,
        path: String,
        uid: Uid,
    ) -> Result<FileSchema, Error> {
        let file_size = uid.get_data_size();
        let chunk_uids = self.get_chunks_of_file(uid).unwrap_or(vec![]);
        let mut chunks = Vec::with_capacity(chunk_uids.len());

        for chunk_uid in chunk_uids.iter() {
            chunks.push(self.get_chunk_by_uid(*chunk_uid)?);
        }

        chunks.sort_by_key(|chunk| chunk.timestamp);

        let (model, last_updated) = match chunks.last() {
            Some(chunk) => (chunk.build_info.model.clone(), chunk.timestamp),
            None => (ChunkBuildInfo::default().model, 0),
        };

        Ok(FileSchema {
            path,
            is_processed: true,
            length: file_size as u64,
            uid,
            chunks: chunks.len(),
            model,
            last_updated: last_updated as u64,
        })
    }
}
