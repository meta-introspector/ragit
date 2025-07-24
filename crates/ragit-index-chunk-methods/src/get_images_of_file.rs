use crate::prelude::*;
use crate::get_chunks_of_file;
use crate::get_chunk_by_uid;

pub fn get_images_of_file(index: &Index, file_uid: Uid) -> Result<Vec<Uid>, ApiError> {
    let chunk_uids = get_chunks_of_file(index, file_uid)?;
    let mut result = HashSet::new();

    for chunk_uid in chunk_uids.into_iter() {
        let chunk = get_chunk_by_uid(index, chunk_uid)?;
        for image in chunk.images.iter() {
            result.insert(*image);
        }
    }

    Ok(result.into_iter().collect())
}
