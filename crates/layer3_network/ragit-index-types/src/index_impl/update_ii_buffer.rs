use crate::index_struct::Index;
use ragit_types::ApiError;
use std::collections::HashMap;
use ragit_types::uid::Uid;

pub fn index_update_ii_buffer(
    _index: &mut Index,
    _ii_buffer: &mut HashMap<String, Vec<Uid>>,
    _chunk_uid: Uid,
) -> Result<(), ApiError> {
    Ok(())
}
