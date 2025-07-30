use crate::index_struct::Index;
use ragit_types::ApiError;
use ragit_types::uid::Uid;

pub async fn index_add_image_description(
    _index: &mut Index,
    _uid: Uid,
) -> Result<(), ApiError> {
    Ok(())
}
