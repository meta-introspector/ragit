use crate::index_struct::Index;
use ragit_types::ApiError;
use std::collections::HashMap;
use ragit_types::uid::Uid;

pub fn index_update_ii_buffer(
    index: &mut Index,
    ii_buffer: &mut HashMap<String, Vec<Uid>>,
    chunk_uid: Uid,
) -> Result<(), ApiError> {
    Ok(())
}
