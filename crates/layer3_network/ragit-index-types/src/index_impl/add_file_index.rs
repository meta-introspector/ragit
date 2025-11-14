use crate::index_struct::Index;
use ragit_types::ApiError;
use ragit_types::uid::Uid;

pub fn index_add_file_index(
    _index: &mut Index,
    _file_uid: Uid,
    _chunk_uids: &[Uid],
) -> Result<(), ApiError> {
        Ok(())
}
