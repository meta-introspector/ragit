use crate::prelude::*;

pub fn check_chunk_by_uid(index: &Index, uid: Uid) -> bool {
    if let Some(chunk_at) = get_uid_path(
        &index.root_dir,
        CHUNK_DIR_NAME,
        uid,
        Some("chunk"),
    ).ok() {
        exists(&chunk_at)
    }

    else {
        false
    }
}
