use crate::index_struct::Index;
use ragit_types::ApiError;
use ragit_types::uid::Uid;

pub fn index_add_file_index(
    index: &mut Index,
    file_uid: Uid,
    chunk_uids: &[Uid],
) -> Result<(), ApiError> {
        Ok(())
}
