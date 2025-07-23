use ragit_types::{Uid, FileSchema};
use anyhow::Error;
use ragit_utils::index::Index;
use ragit_utils::chunk::ChunkBuildInfo;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

pub fn get_file_schema(index: &Index, path: Option<String>, uid: Option<Uid>) -> Result<FileSchema, Error> {
    if let Some(uid) = uid {
        for (path, uid_) in index.processed_files.iter() {
            if uid == *uid_ {
                return get_file_schema_worker(index, path.display().to_string(), uid);
            }
        }
    }

    if let Some(path) = &path {
        if let Some(uid) = index.processed_files.get(Path::new(path)) {
            return get_file_schema_worker(index, path.to_string(), *uid);
        }

        if index.staged_files.contains(&PathBuf::from(path)) {
            return Ok(FileSchema {
                path: path.to_string(),
                is_processed: false,
                ..FileSchema::dummy()
            })
        }
    }

    Err(Error::NoSuchFile { path: path.map(|s| s.into()), uid })
}

pub(crate) fn get_file_schema_worker(index: &Index, path: String, uid: Uid) -> Result<FileSchema, Error> {
    let file_size = uid.get_data_size();
    let chunk_uids = index.get_chunks_of_file(uid).unwrap_or(vec![]);
    let mut chunks = Vec::with_capacity(chunk_uids.len());

    for chunk_uid in chunk_uids.iter() {
        chunks.push(index.get_chunk_by_uid(*chunk_uid)?);
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