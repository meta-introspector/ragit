use crate::index_struct::Index;
use ragit_error::ApiError;
use std::collections::HashMap;
use ragit_types::uid::Uid;

pub fn index_flush_ii_buffer(
    index: &mut Index,
    ii_buffer: HashMap<String, Vec<Uid>>,
) -> Result<(), ApiError> {
    index.flush_ii_buffer(ii_buffer)
}
