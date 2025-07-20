use crate::error::Error;
use ragit_api::Model;

use crate::index::index_struct::Index;

impl Index {
    pub(crate) fn get_model_by_name(&self, name: &str) -> Result<Model, Error> {
        Ok(ragit_api::get_model_by_name(&self.models, name)?.clone())
    }
}
