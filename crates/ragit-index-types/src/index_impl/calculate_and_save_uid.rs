use crate::index_struct::Index;
use ragit_error::ApiError;

pub fn index_calculate_and_save_uid(index: &mut Index) -> Result<(), ApiError> {
    index.calculate_and_save_uid()
}
