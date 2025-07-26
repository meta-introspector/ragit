use crate::index_struct::Index;
use ragit_error::ApiError;

pub fn index_reset_uid(index: &mut Index, save_to_file: bool) -> Result<(), ApiError> {
    index.reset_uid(save_to_file)
}
