use crate::index_struct::Index;
use ragit_types::ApiError;
//use ragit_model::Model;

pub fn index_get_model_by_name(
    _index: &Index,
    _model: &ragit_model::Model,
) -> Result<ragit_model::Model, ApiError> {
        Ok(ragit_model::Model::dummy())
}
