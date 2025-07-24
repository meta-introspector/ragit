use crate::prelude::*;

pub fn get_images_of_file(index: &Index, file_uid: Uid) -> Result<Vec<Uid>, ApiError> {
    let chunk_uids = index.get_chunks_of_file(file_uid)?;
    let mut result = HashSet::new();

    for chunk_uid in chunk_uids.into_iter() {
        let chunk = index.get_chunk_by_uid(chunk_uid)?;
        for image in chunk.images.iter() {
            result.insert(*image);
        }
    }

    Ok(result.into_iter().collect())
}
