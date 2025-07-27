use crate::index_struct::Index;
use ragit_error::ApiError;
use ragit_model::Model;

pub fn index_get_model_by_name(
    index: &Index,
    model: &Model,
) -> Result<Model, ApiError> {
        Ok(Model::dummy())
}
