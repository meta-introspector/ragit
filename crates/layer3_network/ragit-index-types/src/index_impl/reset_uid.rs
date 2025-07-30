use crate::index_struct::Index;
use ragit_types::ApiError;

pub fn index_reset_uid(_index: &mut Index,
		       _save_to_file: bool) -> Result<(), ApiError> {
    Ok(())
}
