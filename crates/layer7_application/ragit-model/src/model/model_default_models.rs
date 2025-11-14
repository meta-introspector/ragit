use super::model_struct::Model;
use crate::model_raw::ModelRaw;

impl Model {
    pub fn default_models() -> Vec<Model> {
        ModelRaw::default_models()
            .iter()
            .map(|model| model.try_into().unwrap())
            .collect()
    }
}
